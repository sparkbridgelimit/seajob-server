项目中间件依赖
redis
本地启动的时候可以通过k8s转发的方式代理到正是环境
kubectl port-forward --namespace default svc/redis-master 6379:6379 &

数据库端口转发
kubectl port-forward --namespace default svc/redis-master 6379:6379 &

本地运行所需的环境变量
DATABASE_URL=postgres://seajob:seajobHm3hd@localhost:6433/seajob;JWT_SECRET_KEY=local-secret-key;REDIS_URL=redis://localhost:6379/0;RUST_BACKTRACE=full