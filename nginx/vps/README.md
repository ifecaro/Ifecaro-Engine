# VPS Nginx Bundles (TLS-ready)

This directory provides copy-ready nginx bundles for VPS deployment.

- `production/`: production edge nginx with `/` + `/db/*` and `/staging/*` path-based proxying.
- `staging/`: standalone staging edge nginx serving root `/` and `/db/*`.

Each bundle contains:

- `nginx.conf`
- `mime.types`
- `conf.d/default.conf`

## Required certificate files

Both bundles expect these files at `/etc/nginx/certs` in the nginx runtime environment:

- `/etc/nginx/certs/cert.pem`
- `/etc/nginx/certs/key.pem`

## Quick copy example

```bash
# production bundle
scp -r nginx/vps/production/* <DEPLOY_USER>@<DEPLOY_HOST>:<DEPLOY_PATH>/nginx/

# staging bundle (if using separate staging edge nginx)
scp -r nginx/vps/staging/* <DEPLOY_USER>@<DEPLOY_HOST>:<STAGING_DEPLOY_PATH>/nginx/
```

After copy, validate and reload the runtime that owns the edge nginx config. For a
host-level VPS nginx service, use:

```bash
sudo nginx -t
sudo systemctl reload nginx
```

For a containerized nginx runtime, use:

```bash
docker exec nginx nginx -t
docker exec nginx nginx -s reload
```

The production VPS bundle assumes the edge nginx process runs on the VPS host and
proxies staging traffic to the staging compose ports via the host loopback address:

```bash
curl -I http://127.0.0.1:18080/
curl -I http://127.0.0.1:18080/staging/
curl -I http://127.0.0.1:18090/api/health
```
