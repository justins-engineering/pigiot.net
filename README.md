# pigiot.net

The front page for pigiot.net

## Development

### Requirements

- [Bun](https://bun.com/get)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/)

### Tailwind CSS

1. Run the following command in the root of the project:
```sh
bun install
```
2. Run the following command in the root of the project to start the Tailwind CSS compiler:

```sh
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --watch
```

### Starting Kratos

Run the following command in the root of your project:

```bash
docker-compose -f docker-compose.yml up --force-recreate
```


### Serving The App for Development

Run the following command in the root of your project to start developing with the default platform:

```sh
dx serve --addr 127.0.0.1 --port 4455 --ssg
```

- Open the web app at http://127.0.0.1:4455
- Open MailSlurper at http://127.0.0.1:4436
- Open [kratos-admin-ui](https://github.com/dhia-gharsallaoui/kratos-admin-ui) at http://127.0.0.1:3000

## Building/Testing

### Bundling

1. Run the following command in the root of the project to compile and minify the Tailwind CSS:

```sh
bunx @tailwindcss/cli -i ./assets/tailwind.css -o ./assets/styling/main.css --minify
```

2. Run the following command in the root of your project to bundle the assets:

```sh
dx build --web --ssg --release --debug-symbols=false
```

### Serving The App

Run the following command in the root of your project to bundle the assets:

```sh
bunx wrangler dev --ip 127.0.0.1 --port 4455
```
