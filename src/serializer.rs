use serialize::json;

use object_builder;

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```
/// use jsonway::{self, Serializer};
///
/// struct Jedi {
///     name: String
/// }
///
/// struct JediSerializer<'a> {
///     jedi: &'a Jedi
/// }
///
/// impl<'a> jsonway::Serializer for JediSerializer<'a> {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, json: &mut jsonway::ObjectBuilder) {
///         json.set("name", self.jedi.name.to_string());
///     }
/// }
///
/// let jedi = Jedi { name: "Saes Rrogon".to_string() };
/// let json = JediSerializer{jedi: &jedi}.serialize();
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "name",
///     ]).unwrap().as_string().unwrap(), 
///     "Saes Rrogon"
/// );
/// ```
pub trait Serializer {

    fn build(&self, &mut object_builder::ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self) -> json::Json {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(&mut bldr);

        bldr.unwrap()
    }
}

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```
/// use jsonway::{self, ObjectSerializer};
///
/// struct Jedi {
///     name: String
/// }
///
/// struct JediSerializer;
///
/// impl jsonway::ObjectSerializer<Jedi> for JediSerializer {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, jedi: &Jedi, json: &mut jsonway::ObjectBuilder) {
///         json.set("name", jedi.name.to_string());
///     }
/// }
///
/// let jedi = Jedi { name: "Saes Rrogon".to_string() };
/// let json = JediSerializer.serialize(&jedi);
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "name",
///     ]).unwrap().as_string().unwrap(), 
///     "Saes Rrogon"
/// );
/// ```
pub trait ObjectSerializer<T> {

    fn build(&self, &T, &mut object_builder::ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T) -> json::Json {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, &mut bldr);
        bldr.unwrap()
    }
}

/// Provides functionality to create custom JSON presenters for your structs.
/// 
/// ## Example 
/// 
/// ```rust
/// use jsonway::{self, ObjectScopeSerializer};
/// 
/// struct User {
///     id: u64,
///     is_admin: bool
/// }
/// 
/// struct Jedi {
///     name: String,
///     secret: String
/// }
/// 
/// struct JediSerializer;
/// 
/// impl jsonway::ObjectScopeSerializer<Jedi, User> for JediSerializer {
///     fn root(&self) -> Option<&str> { Some("jedi") }
///     fn build(&self, jedi: &Jedi, current_user: &User, json: &mut jsonway::ObjectBuilder) {
///         json.set("name", jedi.name.to_string());
/// 
///         if current_user.is_admin {
///             json.set("secret", jedi.secret.to_string());
///         }
///     }
/// }
/// 
/// let jedi = Jedi { 
///     name: "Palpatine".to_string(), 
///     secret: "Dark side".to_string() 
/// };
///
/// let current_user = User { id: 1, is_admin: true };
/// let json = JediSerializer.serialize(&jedi, &current_user);
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "name",
///     ]).unwrap().as_string().unwrap(), 
///     "Palpatine"
/// );
///
/// assert_eq!(
///     json.find_path(&[
///         "jedi",
///         "secret",
///     ]).unwrap().as_string().unwrap(), 
///     "Dark side"
/// );
///
/// ```
pub trait ObjectScopeSerializer<T, S> {

    fn build(&self, &T, &S, &mut object_builder::ObjectBuilder);
    
    #[inline]
    fn root(&self) -> Option<&str> {
        None
    }

    fn serialize(&mut self, obj: &T, scope: &S) -> json::Json {
        let mut bldr = object_builder::ObjectBuilder::new();
        let root = self.root();
        if root.is_some() {
            bldr.root(root.unwrap())
        }
        self.build(obj, scope, &mut bldr);
        bldr.unwrap()
    }
}