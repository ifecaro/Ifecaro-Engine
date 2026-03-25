# VPS Nginx Bundles (TLS-ready)

This directory provides copy-ready nginx bundles for VPS deployment.

- `production/`: production edge nginx with `/` + `/db/*` and `/staging/*` path-based proxying.
- `staging/`: standalone staging edge nginx serving root `/` and `/db/*`.

Each bundle contains:

- `nginx.conf`
- `mime.types`
- `conf.d/default.conf`

## Required certificate files

Both bundles expect these files in `/etc/nginx/certs` inside the container:

- `/etc/nginx/certs/cert.pem`
- `/etc/nginx/certs/key.pem`

## Quick copy example

```bash
# production bundle
scp -r nginx/vps/production/* <DEPLOY_USER>@<DEPLOY_HOST>:<DEPLOY_PATH>/nginx/

# staging bundle (if using separate staging edge nginx)
scp -r nginx/vps/staging/* <DEPLOY_USER>@<DEPLOY_HOST>:<STAGING_DEPLOY_PATH>/nginx/
```

After copy:

```bash
docker exec nginx nginx -t
docker exec nginx nginx -s reload
```
