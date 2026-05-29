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

After copy, validate and reload the runtime that owns the edge nginx config. The
production bundle uses two runtime-specific upstream names:

- `/db/*` proxies to `pocketbase:8090`, the production Docker Compose service
  name used by the containerized nginx runtime.
- `/staging/*` proxies through `host.docker.internal`, so the same `conf.d`
  works in the documented containerized nginx runtime (where Compose maps that
  name to the Docker host gateway) and in host-level nginx deployments.

For a host-level VPS nginx service, make both upstream names resolvable on the
host before validating/reloading nginx. The default compose ports publish
production PocketBase on `127.0.0.1:8090` and the staging services on
`127.0.0.1:18080` / `127.0.0.1:18090`, so map both names to loopback unless
your VPS has separate DNS/hosts entries for them:

```bash
grep -Eq '(^|[[:space:]])host\.docker\.internal([[:space:]]|$)' /etc/hosts || \
  printf '127.0.0.1 host.docker.internal\n' | sudo tee -a /etc/hosts
grep -Eq '(^|[[:space:]])pocketbase([[:space:]]|$)' /etc/hosts || \
  printf '127.0.0.1 pocketbase\n' | sudo tee -a /etc/hosts
sudo nginx -t
sudo systemctl reload nginx
```

For a containerized nginx runtime, use the compose file's existing
`host.docker.internal:host-gateway` mapping and reload the containerized nginx:

```bash
docker exec nginx nginx -t
docker exec nginx nginx -s reload
```

The production VPS bundle expects the production and staging upstreams to be
reachable by the names used in `conf.d/default.conf`:

```bash
curl -I http://pocketbase:8090/api/health
curl -I http://host.docker.internal:18080/
curl -I http://host.docker.internal:18080/staging/
curl -I http://host.docker.internal:18090/api/health
```
