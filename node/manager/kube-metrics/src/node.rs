use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use kube::core::ObjectMeta;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NodeMetricsUsage {
    /// https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#meaning-of-cpu
    pub cpu: Quantity,
    pub memory: Quantity,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NodeMetrics {
    pub metadata: ObjectMeta,
    pub timestamp: String,
    pub window: String,
    pub usage: NodeMetricsUsage,
}

impl k8s_openapi::Resource for NodeMetrics {
    const GROUP: &'static str = "metrics.k8s.io";
    const KIND: &'static str = "node";
    const VERSION: &'static str = "v1beta1";
    const API_VERSION: &'static str = "metrics.k8s.io/v1beta1";
    const URL_PATH_SEGMENT: &'static str = "nodes";
    type Scope = k8s_openapi::NamespaceResourceScope;
}

impl k8s_openapi::Metadata for NodeMetrics {
    type Ty = ObjectMeta;

    fn metadata(&self) -> &Self::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut Self::Ty {
        &mut self.metadata
    }
}
