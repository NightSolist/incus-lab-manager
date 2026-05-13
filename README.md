# README — incus-lab-manager

## Обзор

`incus-lab-manager` — это Rust-клиент для автоматического развёртывания и удаления лабораторных сред на базе [Incus](https://linuxcontainers.org/incus/). Проект является частью ВКР по теме **"Автоматическая синхронизация моделей данных клиентской библиотеки Incus и Rust-приложений на основе гибридной генерации с локальными LLM"**.

Ключевая особенность: все модели данных (DTO) для взаимодействия с Incus API **автоматически сгенерированы** из Go-исходников Incus с помощью гибридного генератора (шаблон + локальная LLM). Вручную не написано ни одной структуры данных.

---

## Архитектура

```
incus-lab-manager/
├── src/
│   ├── main.rs                        # CLI — точка входа (clap)
│   ├── lib.rs                         # Экспорт модулей для тестов
│   ├── client.rs                      # IncusClient — HTTP-клиент (reqwest + mTLS)
│   ├── config.rs                      # Структуры YAML-конфигурации лаборатории
│   ├── lab.rs                         # Deployer — логика deploy/destroy
│   ├── remotes.rs                     # Парсинг image alias (images:alpine/3.21)
│   └── incus/
│       ├── mod.rs                     # Реэкспорт всех DTO
│       ├── custom/
│       │   ├── config_map.rs          # ConfigMap = HashMap<String, String>
│       │   └── devices_map.rs         # DevicesMap = HashMap<String, HashMap<String, String>>
│       └── generated_prototype/       # Автоматически сгенерированные DTO (не редактировать!)
│           ├── mod.rs
│           ├── network_put.rs
│           ├── networks_post.rs
│           ├── profile_put.rs
│           ├── profiles_post.rs
│           ├── storage_pool_put.rs
│           ├── storage_pools_post.rs
│           ├── instance_put.rs
│           ├── instances_post.rs
│           ├── instance_source.rs
│           ├── instance_state_put.rs
│           ├── operation.rs
│           └── ...
├── tests/
│   ├── common/
│   │   └── mod.rs                     # Хелперы: make_client(), short_prefix()
│   ├── integration_lab.rs             # Integration-тест с реальным Incus
│   └── serde_dto.rs                   # Unit-тесты сериализации DTO
├── certs/
│   ├── client.crt                     # Клиентский сертификат (mTLS)
│   └── client.key                     # Приватный ключ
├── demo-lab.yaml                      # Пример лаборатории для демо
├── Cargo.toml
└── .woodpecker.yml                    # CI/CD pipeline (Woodpecker)
```

---

## Сгенерированные DTO

Директория `src/incus/generated_prototype/` содержит **14 Rust-структур**, автоматически сгенерированных из Go-исходников Incus:

| Структура | Назначение | Способ генерации |
|---|---|---|
| `NetworkPut` | Обновление сети | LLM |
| `NetworksPost` | Создание сети | Template |
| `ProfilePut` | Обновление профиля | LLM |
| `ProfilesPost` | Создание профиля | LLM |
| `StoragePoolPut` | Обновление storage pool | LLM |
| `StoragePoolsPost` | Создание storage pool | Template |
| `InstancePut` | Обновление инстанса | LLM |
| `InstancesPost` | Создание инстанса | Template |
| `InstanceSource` | Источник образа | Template |
| `InstanceStatePut` | Управление состоянием | Template |
| `Operation` | Асинхронная операция | Template |
| `Network` | Модель сети | Template |
| `Profile` | Модель профиля | Template |
| `StoragePool` | Модель storage pool | Template |

> ⚠️ Файлы в `generated_prototype/` **не редактировать вручную**. Они обновляются автоматически генератором при изменениях в Go-репозитории Incus.

---

## Требования

### Системные

- **ОС**: Linux (Ubuntu 22.04+ / Debian 12+)
- **Rust**: 1.83+
- **Incus**: 6.x (установлен и инициализирован локально)

### Установка Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version   # должно быть 1.83+
```

### Установка Incus

```bash
# Добавить репозиторий Zabbly
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

# Добавить пользователя в группы
sudo usermod -aG incus-admin $USER
sudo usermod -aG incus $USER
newgrp incus-admin

# Проверить версию
incus --version   # должно быть 6.x
```

### Инициализация Incus

```bash
sudo incus admin init
```

Ответы на вопросы мастера:

```
Would you like to use clustering?          → no
Do you want to configure a storage pool?   → yes
Name of the storage pool:                  → default
Storage backend:                           → dir
Create a new local network bridge?         → yes
Bridge name:                               → incusbr0
IPv4 address:                              → auto
IPv6 address:                              → none
Server available over the network?         → yes
Address to bind to:                        → 127.0.0.1
Port:                                      → 8443
Update images automatically?               → yes
Print YAML preseed?                        → no
```

---

## Установка и настройка

### 1. Клонировать репозиторий

```bash
git clone https://github.com/NightSolist/incus-lab-manager.git
cd incus-lab-manager
```

### 2. Сгенерировать клиентский сертификат

Incus использует mTLS (взаимный TLS). Нужно создать сертификат для аутентификации клиента:

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
```

Проверить:

```bash
incus config trust list
```

Должен появиться `incus-lab-manager` в списке.

### 4. Проверить аутентификацию

```bash
curl --cert certs/client.crt \
     --key certs/client.key \
     -k -s https://127.0.0.1:8443/1.0 | \
     python3 -c "import json,sys; print('auth:', json.load(sys.stdin)['metadata']['auth'])"
```

Ожидаемый вывод:

```
auth: trusted
```

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

### CLI команды

```
incus-lab-manager <COMMAND>

COMMANDS:
  deploy   Развернуть лабораторию из YAML-файла
  destroy  Удалить лабораторию, описанную в YAML-файле
  start    Запустить конкретный инстанс
  stop     Остановить конкретный инстанс
  delete   Удалить конкретный инстанс
  info     Проверить подключение к Incus
```

### Проверка подключения

```bash
cargo run -- info
```

Ожидаемый вывод:

```
📡 Checking connection...
✅ Server Info: {"type":"sync","status":"Success",...}
```

### Развернуть лабораторию

```bash
cargo run -- deploy --file demo-lab.yaml
```

Ожидаемый вывод:

```
🚀 Deploying lab: demo-lab (Полная демо-лаборатория...)
💾 Deploying storage pool 'demo-pool'
✅ Created storage pool demo-pool
🌐 Deploying network 'demo-net'
✅ Created network demo-net
👤 Deploying profile 'demo-profile'
✅ Created profile demo-profile
📦 Deploying demo-web (container)
✅ Created demo-web
📦 Deploying demo-db (container)
✅ Created demo-db
✅ Lab deployed successfully!
```

### Проверить созданные ресурсы

```bash
incus list
incus network list
incus storage list
incus profile list
```

### Удалить лабораторию

```bash
cargo run -- destroy --file demo-lab.yaml
```

Ожидаемый вывод:

```
🔥 Destroying lab: demo-lab
   🗑️  Deleting instance: demo-web
   🗑️  Deleting instance: demo-db
   🗑️  Deleting profile: demo-profile
   🗑️  Deleting network: demo-net
   🗑️  Deleting storage pool: demo-pool
✅ Lab destroyed.
```

---

## Формат YAML-конфигурации

```yaml
name: my-lab
description: "Описание лаборатории"

# Storage pools (опционально)
storage_pools:
  - name: my-pool
    driver: dir                        # dir, zfs, btrfs, lvm
    description: "Описание пула"
    source: /path/to/storage           # опционально, для dir-driver

# Сети (опционально)
networks:
  - name: my-net
    ipv4: 10.50.0.1/24                 # IP шлюза / "auto" / "none"
    ipv6: none                         # IP шлюза / "auto" / "none"
    dns_domain: lab.local              # опционально

# Профили (опционально)
profiles:
  - name: my-profile
    description: "Описание профиля"
    network: my-net                    # имя сети из раздела networks
    storage: my-pool                   # имя пула из раздела storage_pools
    config:
      limits.memory: 512MB             # любые Incus-опции
      limits.cpu: "2"

# Инстансы (контейнеры / VM)
instances:
  - name: my-container
    type: container                    # container / virtual-machine
    image: images:alpine/3.21         # образ с remote или локальный алиас
    profiles:
      - my-profile
    start: true                        # запустить после создания
    config:
      limits.memory: 256MB            # переопределение конфига
```

### Примеры значений image

```yaml
image: images:alpine/3.21          # из публичного репозитория images:
image: images:ubuntu/24.04          # Ubuntu из репозитория images:
image: alpine-3.21                  # локальный алиас (после incus image copy)
```

---

## Пример демо-лаборатории

Файл `demo-lab.yaml` в корне репозитория:

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

Проверяют корректность serde-аннотаций сгенерированных структур. **Не требуют Incus**.

```bash
cargo test --test serde_dto -- --nocapture
```

Ожидаемый вывод:

```
running 8 tests
test test_network_put_round_trip ... ok
test test_network_put_omitempty ... ok
test test_networks_post_serialization ... ok
test test_profile_put_round_trip ... ok
test test_profiles_post_serialization ... ok
test test_storage_pool_put_round_trip ... ok
test test_instance_source_serialization ... ok
test test_instances_post_serialization ... ok

test result: ok. 8 passed; 0 failed; finished in 0.00s
```

### Integration-тест с реальным Incus

Полный сценарий: создаёт storage pool → сеть → профиль → инстанс через сгенерированные DTO, затем удаляет всё. **Требует запущенного Incus**.

```bash
cargo test --test integration_lab -- --nocapture --test-threads=1
```

Ожидаемый вывод:

```
running 1 test

══════════════════════════════════════════════
🧪 Тест полного цикла деплоя лаборатории
   Префикс ресурсов: t21562
══════════════════════════════════════════════

📦 [1/5] Создаём storage pool 't21562-pool'...
   ✓ Storage pool создан
🌐 [2/5] Создаём сеть 't21562-net'...
   ✓ Сеть создана
👤 [3/5] Создаём профиль 't21562-prof'...
   ✓ Профиль создан
🚀 [4/5] Создаём инстанс 't21562-inst'...
   ✓ Инстанс создан
🔍 [5/5] Все ресурсы созданы успешно
🧹 Очистка ресурсов...
   ✓ Очистка завершена

✅ Тест пройден: полный сценарий деплоя работает

test result: ok. 1 passed; 0 failed; finished in 3.09s
```

### Запустить все тесты

```bash
cargo test -- --nocapture --test-threads=1
```

---

## CI/CD

Проект использует **self-hosted Woodpecker CI/CD**. Pipeline запускается автоматически при публикации Pull Request генератором или вручную через UI.

### Шаги pipeline

| Шаг | Образ | Что делает |
|---|---|---|
| `cargo-check` | `rust:1.83` | Компиляционная валидация всех DTO |
| `cargo-fmt` | `rust:1.83` | Проверка форматирования |
| `cargo-test-unit` | `rust:1.83` | Unit-тесты сериализации DTO (8 тестов) |
| `notify-engineer` | `deblan/woodpecker-email` | Email-уведомление при успехе |
| `notify-failure` | `deblan/woodpecker-email` | Email-уведомление при ошибке |

> Integration-тесты с реальным Incus **не входят в CI** — они выполняются локально, так как в Docker-окружении Woodpecker нет Incus.

---

## Устранение неполадок

### `auth: untrusted`

Сертификат не добавлен в trust store:

```bash
incus config trust list
# Если пусто:
incus config trust add-certificate certs/client.crt --name incus-lab-manager
```

### `Connection refused` при запуске тестов

Incus не запущен:

```bash
sudo systemctl start incus
sudo systemctl status incus
```

### `Network interface is too long`

Имя сети превышает 15 символов (ограничение ядра Linux). Используйте короткие имена:

```yaml
# Плохо (19 символов):
name: test-1778521450-net

# Хорошо (10 символов):
name: demo-net
```

### Образ не найден при `deploy`

```bash
# Проверить локальные образы
incus image list

# Скачать Alpine 3.21
incus image copy images:alpine/3.21 local: --alias alpine-3.21
```

### Ресурсы остались после упавшего теста

```bash
# Посмотреть что висит
incus list
incus network list
incus profile list
incus storage list

# Удалить вручную
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

Этот репозиторий является **целевым** для системы автоматической синхронизации. Общий flow:

```
Go-репозиторий Incus (изменение структуры)
    ↓
Генератор (Python + Jinja2 + LLM Qwen2.5-Coder)
    ↓
Pull Request в этот репозиторий
    ↓  (обновление src/incus/generated_prototype/)
Woodpecker CI: cargo-check → cargo-fmt → cargo-test-unit
    ↓
Email-уведомление инженеру
```

Сгенерированные файлы в `src/incus/generated_prototype/` **никогда не редактируются вручную** — только через генератор.

---

## Лицензия

MIT — см. [LICENSE](LICENSE)