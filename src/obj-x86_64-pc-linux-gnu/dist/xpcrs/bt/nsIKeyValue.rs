//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/kvstore/nsIKeyValue.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIKeyValueService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getOrCreate (in nsIKeyValueDatabaseCallback callback, in AUTF8String path, in AUTF8String name); */
                    Method {
                        name: "GetOrCreate",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueDatabaseCallback" }, Param { name: "path", ty: "*const ::nsstring::nsACString" }, Param { name: "name", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueDatabase",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void put (in nsIKeyValueVoidCallback callback, in AUTF8String key, in nsIVariant value); */
                    Method {
                        name: "Put",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVoidCallback" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void writeMany (in nsIKeyValueVoidCallback callback, in Array<nsIKeyValuePair> pairs); */
                    Method {
                        name: "WriteMany",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVoidCallback" }, Param { name: "pairs", ty: "*const thin_vec::ThinVec<RefPtr<nsIKeyValuePair>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void get (in nsIKeyValueVariantCallback callback, in AUTF8String key, [optional] in nsIVariant defaultValue); */
                    Method {
                        name: "Get",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVariantCallback" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "defaultValue", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void has (in nsIKeyValueVariantCallback callback, in AUTF8String key); */
                    Method {
                        name: "Has",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVariantCallback" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void delete (in nsIKeyValueVoidCallback callback, in AUTF8String key); */
                    Method {
                        name: "Delete",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVoidCallback" }, Param { name: "key", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (in nsIKeyValueVoidCallback callback); */
                    Method {
                        name: "Clear",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueVoidCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void enumerate (in nsIKeyValueEnumeratorCallback callback, [optional] in AUTF8String fromKey, [optional] in AUTF8String toKey); */
                    Method {
                        name: "Enumerate",
                        params: &[Param { name: "callback", ty: "*const nsIKeyValueEnumeratorCallback" }, Param { name: "fromKey", ty: "*const ::nsstring::nsACString" }, Param { name: "toKey", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValuePair",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String key; */
                    Method {
                        name: "GetKey",
                        params: &[Param { name: "aKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIVariant value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut *const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueEnumerator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* bool hasMoreElements (); */
                    Method {
                        name: "HasMoreElements",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIKeyValuePair getNext (); */
                    Method {
                        name: "GetNext",
                        params: &[Param { name: "_retval", ty: "*mut *const nsIKeyValuePair" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueDatabaseCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resolve (in nsIKeyValueDatabase database); */
                    Method {
                        name: "Resolve",
                        params: &[Param { name: "database", ty: "*const nsIKeyValueDatabase" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reject (in AUTF8String message); */
                    Method {
                        name: "Reject",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueEnumeratorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resolve (in nsIKeyValueEnumerator enumerator); */
                    Method {
                        name: "Resolve",
                        params: &[Param { name: "enumerator", ty: "*const nsIKeyValueEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reject (in AUTF8String message); */
                    Method {
                        name: "Reject",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValuePairCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resolve (in nsIKeyValuePair pair); */
                    Method {
                        name: "Resolve",
                        params: &[Param { name: "pair", ty: "*const nsIKeyValuePair" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reject (in AUTF8String message); */
                    Method {
                        name: "Reject",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueVariantCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resolve (in nsIVariant result); */
                    Method {
                        name: "Resolve",
                        params: &[Param { name: "result", ty: "*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reject (in AUTF8String message); */
                    Method {
                        name: "Reject",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIKeyValueVoidCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resolve (); */
                    Method {
                        name: "Resolve",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void reject (in AUTF8String message); */
                    Method {
                        name: "Reject",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

