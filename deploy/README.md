# Deployment

The Dockerfile builds the Dioxus web bundle and serves it with nginx.

```bash
docker build -f deploy/Dockerfile -t mw-boilerplate:local .
docker run --rm -p 8080:80 mw-boilerplate:local
```

Open http://localhost:8080. Coolify can deploy using this Dockerfile. Verify the generated Dioxus output path and deep-link routing behavior against the pinned CLI before production rollout. This starter has no API server or authentication service.
