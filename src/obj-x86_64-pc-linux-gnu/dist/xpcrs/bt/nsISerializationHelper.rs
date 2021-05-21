//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISerializationHelper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISerializationHelper",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString serializeToString (in nsISerializable serializable); */
                    Method {
                        name: "SerializeToString",
                        params: &[Param { name: "serializable", ty: "*const nsISerializable" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports deserializeObject (in ACString input); */
                    Method {
                        name: "DeserializeObject",
                        params: &[Param { name: "input", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

