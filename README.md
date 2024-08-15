项目中间件依赖
redis
本地启动的时候可以通过k8s转发的方式代理到正是环境
kubectl port-forward --namespace default svc/redis-master 6379:6379 &