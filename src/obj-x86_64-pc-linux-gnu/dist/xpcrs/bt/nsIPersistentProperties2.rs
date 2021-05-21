//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPersistentProperties2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPropertyElement",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute AUTF8String key; */
                    Method {
                        name: "GetKey",
                        params: &[Param { name: "aKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetKey",
                        params: &[Param { name: "aKey", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetValue",
                        params: &[Param { name: "aValue", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIPersistentProperties",
            base: Some("nsIProperties"),
            methods: Ok(&[
                    /* void load (in nsIInputStream input); */
                    Method {
                        name: "Load",
                        params: &[Param { name: "input", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void save (in nsIOutputStream output, in AUTF8String header); */
                    Method {
                        name: "Save",
                        params: &[Param { name: "output", ty: "*const nsIOutputStream" }, Param { name: "header", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISimpleEnumerator enumerate (); */
                    Method {
                        name: "Enumerate",
                        params: &[Param { name: "_retval", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getStringProperty (in AUTF8String key); */
                    Method {
                        name: "GetStringProperty",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString setStringProperty (in AUTF8String key, in AString value); */
                    Method {
                        name: "SetStringProperty",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

