/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct AutoscalingV1ApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AutoscalingV1ApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AutoscalingV1ApiClient<C> {
        AutoscalingV1ApiClient {
            configuration: configuration,
        }
    }
}

pub trait AutoscalingV1Api {
    fn create_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1Status, Error = Error>>;
    fn delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::MetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::MetaV1Status, Error = Error>>;
    fn get_autoscaling_v1_api_resources(&self, ) -> Box<Future<Item = ::models::MetaV1ApiResourceList, Error = Error>>;
    fn list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscalerList, Error = Error>>;
    fn list_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscalerList, Error = Error>>;
    fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::MetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: ::models::MetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>>;
    fn watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>>;
    fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>>;
    fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list(&self, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>>;
}


impl<C: hyper::client::Connect>AutoscalingV1Api for AutoscalingV1ApiClient<C> {
    fn create_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers{}", configuration.base_path, query, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers{}", configuration.base_path, query, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::MetaV1DeleteOptions, pretty: &str, grace_period_seconds: i32, orphan_dependents: bool, propagation_policy: &str) -> Box<Future<Item = ::models::MetaV1Status, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Delete;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("gracePeriodSeconds", &grace_period_seconds.to_string())
            .append_pair("orphanDependents", &orphan_dependents.to_string())
            .append_pair("propagationPolicy", &propagation_policy.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1Status, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn get_autoscaling_v1_api_resources(&self, ) -> Box<Future<Item = ::models::MetaV1ApiResourceList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/apis/autoscaling/v1/", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1ApiResourceList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscalerList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/horizontalpodautoscalers{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscalerList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn list_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, namespace: &str, pretty: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscalerList, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers{}", configuration.base_path, query, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscalerList, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::MetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: ::models::MetaV1Patch, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Patch;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, pretty: &str, exact: bool, export: bool) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("exact", &exact.to_string())
            .append_pair("export", &export.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(&self, name: &str, namespace: &str, body: ::models::AutoscalingV1HorizontalPodAutoscaler, pretty: &str) -> Box<Future<Item = ::models::AutoscalingV1HorizontalPodAutoscaler, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("pretty", &pretty.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::AutoscalingV1HorizontalPodAutoscaler, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces(&self, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/watch/horizontalpodautoscalers{}", configuration.base_path, query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1WatchEvent, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(&self, name: &str, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name}{}", configuration.base_path, query, name=name, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1WatchEvent, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list(&self, namespace: &str, field_selector: &str, include_uninitialized: bool, label_selector: &str, pretty: &str, resource_version: &str, timeout_seconds: i32, watch: bool) -> Box<Future<Item = ::models::MetaV1WatchEvent, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("fieldSelector", &field_selector.to_string())
            .append_pair("includeUninitialized", &include_uninitialized.to_string())
            .append_pair("labelSelector", &label_selector.to_string())
            .append_pair("pretty", &pretty.to_string())
            .append_pair("resourceVersion", &resource_version.to_string())
            .append_pair("timeoutSeconds", &timeout_seconds.to_string())
            .append_pair("watch", &watch.to_string())
            .finish();
        let uri_str = format!("{}/apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers{}", configuration.base_path, query, namespace=namespace);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::MetaV1WatchEvent, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
