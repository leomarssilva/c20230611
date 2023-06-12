from diagrams import Diagram, Cluster, Edge

from diagrams.generic.network import Firewall
from diagrams.generic.os import RedHat
from diagrams.onprem.logging import Loki
from diagrams.onprem.monitoring import Grafana, Prometheus
from diagrams.onprem.compute import Server
from diagrams.onprem.network import Nginx

with Diagram("", show=False):
    firewall = Firewall()

    server = RedHat("Server")
    
    firewall >> Edge(label="HTTPS") >> server

    with Cluster("Monitoring"):
        loki = Loki("Logs")
        prometheus = Prometheus("Metrics")

    grafana = Grafana("Dashboards / Alerts")

    loki >> grafana
    prometheus >> grafana
    
    server >> loki
    prometheus >> server

    # server >> Edge() >> backend
