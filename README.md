# README — incus-lab-manager

## Обзор

`incus-lab-manager` — Rust-клиент для автоматического развёртывания и удаления лабораторных сред на базе [Incus](https://linuxcontainers.org/incus/). Проект разработан в рамках ВКР по теме **«Автоматическая синхронизация моделей данных клиентской библиотеки Incus и Rust-приложений на основе гибридной генерации с локальными LLM»**.

Ключевая особенность: все модели данных (DTO) для взаимодействия с Incus API **автоматически синхронизируются** из Go-исходников Incus генератором `dto_converter`. Вручную не написано ни одной DTO-структуры — все они генерируются по шаблонам или восстанавливаются механизмом LLM repair при невозможности шаблонной генерации.

---

## Архитектура репозитория

```
incus-lab-manager/
├── src/
│   ├── main.rs                        # CLI (clap) — точка входа
│   ├── lib.rs                         # Экспорт модулей
│   ├── client.rs                      # IncusClient — HTTP-клиент (reqwest + mTLS)
│   ├── config.rs                      # Структуры YAML-конфигурации лаборатории
│   ├── lab.rs                         # Deployer — логика deploy/destroy
│   ├── remotes.rs                     # Парсинг image alias (images:alpine/3.21)
│   └── incus/
│       ├── mod.rs                     # Реэкспорт всех DTO
│       └── generated_prototype/       # Автоматически сгенерированные DTO
│           ├── mod.rs                 # Реэкспорт через pub mod / pub use
│           ├── config_map.rs          # ConfigMap + кастомные deserializers
│           ├── devices_map.rs         # DevicesMap + кастомные deserializers
│           ├── network.rs
│           ├── network_put.rs
│           ├── networks_post.rs
│           ├── profile.rs
│           ├── profile_put.rs
│           ├── profiles_post.rs
│           ├── storage_pool.rs
│           ├── storage_pool_put.rs
│           ├── storage_pools_post.rs
│           ├── instance_put.rs
│           ├── instance_source.rs
│           ├── instance_state_put.rs
│           ├── instances_post.rs
│           ├── instance_type.rs
│           ├── operation.rs
│           └── status_code.rs
├── tests/
│   ├── common/mod.rs                  # Хелперы: make_client(), short_prefix()
│   ├── integration_lab.rs             # Integration-тест с реальным Incus
│   └── serde_dto.rs                   # Unit-тесты сериализации DTO
├── certs/
│   ├── client.crt                     # Клиентский сертификат (mTLS)
│   └── client.key                     # Приватный ключ
├── demo-lab.yaml                      # Пример лаборатории
├── Cargo.toml
└── .woodpecker.yml                    # CI/CD pipeline потребителя
```

---

## Сгенерированные DTO

Каталог `src/incus/generated_prototype/` содержит **19 файлов**, полностью сгенерированных из Go-структур пакета `shared/api/` репозитория Incus:

| Категория | Сущности | Способ генерации |
|---|---|---|
| Псевдонимы | `ConfigMap`, `DevicesMap` | специализированные шаблоны Jinja2 |
| Перечисления | `InstanceType`, `StatusCode` | шаблон `enum.rs.j2` |
| Структуры | `Network`, `NetworkPut`, `NetworksPost`, `Profile`, `ProfilePut`, `ProfilesPost`, `StoragePool`, `StoragePoolPut`, `StoragePoolsPost`, `InstancePut`, `InstancesPost`, `InstanceSource`, `InstanceStatePut`, `Operation` | шаблон `struct.rs.j2`, многоволновое разрешение зависимостей |

Для типов `ConfigMap` и `DevicesMap` дополнительно генерируются вспомогательные функции десериализации (`deserialize_config_map`, `deserialize_option_config_map`, `deserialize_devices_map`, `deserialize_option_devices_map`), воспроизводящие поведение Go-реализации `UnmarshalJSON`: в JSON допускаются значения типов `bool`, `number` и `null`, которые автоматически приводятся к строкам.

> ⚠️ Файлы в `generated_prototype/` **не редактируются вручную**. Они полностью перезаписываются при каждом запуске генератора.

---

## Требования

### Системные

- ОС: Linux (Ubuntu 22.04+ / Debian 12+)
- Rust: 1.83+
- Incus: 6.x (установлен и инициализирован)

### Установка Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version   # 1.83+
```

### Установка Incus

```bash
sudo mkdir -p /etc/apt/keyrings/
sudo curl -fsSL https://pkgs.zabbly.com/key.asc -o /etc/apt/keyrings/zabbly.asc

sudo sh -c 'cat <<EOF > /etc/apt/sources.list.d/zabbly-incus-stable.sources
Enabled: yes
Types: deb
URIs: https://pkgs.zabbly.com/incus/stable
Suites: $(. /etc/os-release && echo ${VERSION_CODENAME})
Components: main
Architectures: $(dpkg --print-architecture)
Signed-By: /etc/apt/keyrings/zabbly.asc
EOF'

sudo apt update
sudo apt install -y incus incus-tools

sudo usermod -aG incus-admin $USER
sudo usermod -aG incus $USER
newgrp incus-admin

incus --version
```

### Инициализация Incus

```bash
sudo incus admin init
```

Параметры мастера:

```
Use clustering?                       → no
Configure a storage pool?             → yes
Storage pool name:                    → default
Storage backend:                      → dir
Create a new local network bridge?    → yes
Bridge name:                          → incusbr0
IPv4 address:                         → auto
IPv6 address:                         → none
Server available over the network?    → yes
Address to bind to:                   → 127.0.0.1
Port:                                 → 8443
Update images automatically?          → yes
Print YAML preseed?                   → no
```

---

## Установка и настройка

### 1. Клонировать репозиторий

```bash
git clone https://github.com/NightSolist/incus-lab-manager.git
cd incus-lab-manager
```

### 2. Сгенерировать клиентский сертификат

```bash
mkdir -p certs

openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
  -nodes \
  -keyout certs/client.key \
  -out certs/client.crt \
  -subj "/CN=incus-lab-manager-client"

chmod 600 certs/client.key
chmod 644 certs/client.crt
```

### 3. Добавить сертификат в trust store Incus

```bash
incus config trust add-certificate certs/client.crt --name incus-lab-manager
incus config trust list
```

### 4. Проверить аутентификацию

```bash
curl --cert certs/client.crt \
     --key certs/client.key \
     -k -s https://127.0.0.1:8443/1.0 | \
     python3 -c "import json,sys; print('auth:', json.load(sys.stdin)['metadata']['auth'])"
```

Ожидаемо: `auth: trusted`.

### 5. Скачать образ Alpine

```bash
incus image copy images:alpine/3.21 local: --alias alpine-3.21
incus image list
```

### 6. Собрать проект

```bash
cargo build
```

---

## Использование

### CLI

```
incus-lab-manager <COMMAND>

COMMANDS:
  deploy   Развернуть лабораторию из YAML-файла
  destroy  Удалить лабораторию, описанную в YAML-файле
  start    Запустить инстанс
  stop     Остановить инстанс
  delete   Удалить инстанс
  info     Проверить подключение к Incus
```

### Проверка подключения

```bash
cargo run -- info
```

### Развернуть лабораторию

```bash
cargo run -- deploy --file demo-lab.yaml
```

### Удалить лабораторию

```bash
cargo run -- destroy --file demo-lab.yaml
```

---

## Формат YAML-конфигурации

```yaml
name: my-lab
description: "Описание лаборатории"

storage_pools:
  - name: my-pool
    driver: dir
    description: "Описание пула"
    source: /path/to/storage          # опционально

networks:
  - name: my-net
    ipv4: 10.50.0.1/24                # CIDR / auto / none
    ipv6: none
    dns_domain: lab.local             # опционально

profiles:
  - name: my-profile
    description: "Описание профиля"
    network: my-net
    storage: my-pool
    config:
      limits.memory: 512MB
      limits.cpu: "2"

instances:
  - name: my-container
    type: container                   # container / virtual-machine
    image: images:alpine/3.21
    profiles:
      - my-profile
    start: true
    config:
      limits.memory: 256MB
```

---

## Пример демо-лаборатории

`demo-lab.yaml`:

```yaml
name: demo-lab
description: "Полная демо-лаборатория: storage pool + сеть + профиль + 2 контейнера"

storage_pools:
  - name: demo-pool
    driver: dir
    description: "Demo storage pool (dir-based)"

networks:
  - name: demo-net
    ipv4: 10.50.0.1/24
    ipv6: none
    dns_domain: demo.local

profiles:
  - name: demo-profile
    description: "Demo profile with network and storage"
    network: demo-net
    storage: demo-pool

instances:
  - name: demo-web
    type: container
    image: images:alpine/3.21
    profiles:
      - demo-profile
    start: true

  - name: demo-db
    type: container
    image: images:alpine/3.21
    profiles:
      - demo-profile
    start: true
```

---

## Тестирование

### Unit-тесты сериализации DTO

Проверяют корректность serde-аннотаций сгенерированных структур. Не требуют запущенного Incus.

```bash
cargo test --test serde_dto -- --nocapture
```

### Integration-тест с реальным Incus

Полный сценарий: создаёт storage pool → сеть → профиль → инстанс через сгенерированные DTO, затем удаляет всё. Требует запущенного локального Incus.

```bash
cargo test --test integration_lab -- --nocapture --test-threads=1
```

### Все тесты сразу

```bash
cargo test -- --nocapture --test-threads=1
```

---

## CI/CD

Проект использует **self-hosted Woodpecker CI/CD**. Pipeline потребителя запускается:

- автоматически — по триггеру из pipeline генератора `dto_converter` через Woodpecker REST API после публикации Pull Request;
- вручную — через Woodpecker UI.

### Шаги pipeline (`.woodpecker.yml`)

| Шаг | Образ | Назначение |
|---|---|---|
| `cargo-check` | `rust:1.83` | `cargo check --all-targets` |
| `cargo-fmt-check` | `rust:1.83` | `cargo fmt --all -- --check` |
| `cargo-test-serde` | `rust:1.83` | `cargo test --test serde_dto` |
| `notify-success` | `python:3.12-slim` | Email-уведомление при успехе через `smtplib` |
| `notify-failure` | `python:3.12-slim` | Email-уведомление при ошибке через `smtplib` |

Integration-тесты с реальным Incus в CI не запускаются — они выполняются локально, поскольку в Docker-окружении Woodpecker нет работающего Incus.

---

## Устранение неполадок

### `auth: untrusted`

Сертификат не добавлен в trust store:

```bash
incus config trust list
incus config trust add-certificate certs/client.crt --name incus-lab-manager
```

### `Connection refused` при тестах

Incus не запущен:

```bash
sudo systemctl start incus
sudo systemctl status incus
```

### `Network interface is too long`

Имя сети превышает 15 символов. Используйте короткие имена.

### Образ не найден

```bash
incus image list
incus image copy images:alpine/3.21 local: --alias alpine-3.21
```

### Очистка зависших ресурсов после упавшего теста

```bash
incus list
incus network list
incus profile list
incus storage list

incus delete <имя-инстанса> --force
incus profile delete <имя-профиля>
incus network delete <имя-сети>
incus storage delete <имя-пула>
```

---

## Зависимости

```toml
[dependencies]
tokio       = { version = "1", features = ["full"] }
serde       = { version = "1", features = ["derive"] }
serde_yaml  = "0.9"
serde_json  = "1"
reqwest     = { version = "0.11", features = ["json", "rustls-tls"], default-features = false }
anyhow      = "1.0"
chrono      = { version = "0.4", features = ["serde"] }
clap        = { version = "4.4", features = ["derive"] }

[dev-dependencies]
scopeguard  = "1.2"
```

---

## Связь с генератором DTO

Этот репозиторий является **целевым** для системы автоматической синхронизации `dto_converter`. Общий поток работы:

```
Go-репозиторий Incus (изменение API)
    ↓
dto_converter (Python + Jinja2 + Ollama / qwen2.5-coder:1.5b)
    ↓ template-first генерация
    ↓ многоволновое разрешение зависимостей
    ↓ LLM repair при необходимости
Pull Request в incus-lab-manager
    ↓
Триггер lab CI через Woodpecker REST API
    ↓
cargo check → cargo fmt --check → cargo test --test serde_dto
    ↓
Email-уведомление инженеру
```

Файлы в `src/incus/generated_prototype/` обновляются только генератором — не вручную.