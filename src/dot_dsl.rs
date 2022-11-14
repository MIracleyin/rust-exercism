pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::edge::Edge;
    use self::graph_items::node::Node;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.clone().to_vec();

            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.clone().to_vec();

            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
            attrs.iter().for_each(|&(name, value)| {
                self.attrs.insert(name.to_string(), value.to_string());
            });

            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().filter(|n| n.name == name).nth(0)
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
    
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }
    
            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Edge {
                        src: src.into(),
                        dst: dst.into(),
                        attrs: HashMap::new(),
                    }
                }
    
                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|&(name, value)| {
                        self.attrs.insert(name.into(), value.into());
                    });
    
                    self
                }
            }
        }
    
        pub mod node {
            use std::collections::HashMap;
    
            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
    
                attrs: HashMap<String, String>,
            }
    
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.into(),
                        attrs: HashMap::new(),
                    }
                }
    
                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Self {
                    attrs.iter().for_each(|&(name, value)| {
                        self.attrs.insert(name.into(), value.into());
                    });
    
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|v| v.as_ref())
                }
            }
        }
    }
}




use graph::Graph;
use graph::graph_items::edge::Edge;
use graph::graph_items::node::Node;

#[test]
fn test_empty_graph() {
    let graph = Graph::new();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());
}

#[test]
//#[ignore]
fn test_graph_with_one_node() {
    let nodes = vec![Node::new("a")];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.nodes, vec![Node::new("a")]);
}

#[test]
//#[ignore]
fn test_graph_with_one_node_with_keywords() {
    let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(
        graph.nodes,
        vec![Node::new("a").with_attrs(&[("color", "green")])]
    );
}

#[test]
//#[ignore]
fn test_graph_with_one_edge() {
    let edges = vec![Edge::new("a", "b")];

    let graph = Graph::new().with_edges(&edges);

    assert!(graph.nodes.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
}

#[test]
//#[ignore]
fn test_graph_with_one_attribute() {
    let graph = Graph::new().with_attrs(&[("foo", "1")]);

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
    };

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
//#[ignore]
fn test_graph_with_attributes() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
        "title".to_string() => "Testing Attrs".to_string(),
        "bar".to_string() => "true".to_string(),
    };

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.nodes,
        vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ]
    );

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ]
    );

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
//#[ignore]
fn test_graph_stores_attributes() {
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
    let graph = Graph::new().with_nodes(
        &['a', 'b', 'c']
            .iter()
            .enumerate()
            .map(|(i, n)| Node::new(&n.to_string()).with_attrs(&attributes[i..i + 1]))
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        graph
            .get_node("c")
            .expect("node must be stored")
            .get_attr("bim"),
        Some("bef")
    );
}
