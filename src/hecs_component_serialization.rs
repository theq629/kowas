#[macro_export]
macro_rules! serialize_components {
    ( $( $component:ident ), *) => {
        #[derive(serde::Serialize, serde::Deserialize)]
        enum ComponentId {
            $(
            $component,
            )*
        }

        pub struct ComponentSerializationContext;

        impl hecs::serialize::SerializeContext for ComponentSerializationContext {
            fn serialize_entity<S>(
                &mut self,
                entity: hecs::EntityRef<'_>,
                map: &mut S,
            ) -> Result<(), S::Error>
            where
                S: serde::ser::SerializeMap
            {
                $(
                hecs::serialize::try_serialize::<$component, _, _>(&entity, &ComponentId::$component, map)?;
                )*
                Ok(())
            }
        }

        impl hecs::serialize::DeserializeContext for ComponentSerializationContext {
            fn deserialize_entity<'a, M>(
                &mut self,
                mut map: M,
                entity: &mut hecs::EntityBuilder,
            ) -> Result<(), M::Error>
            where
                M: serde::de::MapAccess<'a>,
            {
                while let Some(key) = map.next_key()? {
                    match key {
                        $(
                        ComponentId::$component => {
                            entity.add::<$component>(map.next_value()?);
                        },
                        )*
                    }
                }
                Ok(())
            }
        }
    };
}
