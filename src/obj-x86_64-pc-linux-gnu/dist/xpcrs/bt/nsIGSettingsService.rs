//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGSettingsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIGSettingsCollection",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setString (in AUTF8String key, in AUTF8String value); */
                    Method {
                        name: "SetString",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setBoolean (in AUTF8String key, in boolean value); */
                    Method {
                        name: "SetBoolean",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setInt (in AUTF8String key, in long value); */
                    Method {
                        name: "SetInt",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getString (in AUTF8String key); */
                    Method {
                        name: "GetString",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean getBoolean (in AUTF8String key); */
                    Method {
                        name: "GetBoolean",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* long getInt (in AUTF8String key); */
                    Method {
                        name: "GetInt",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIArray getStringList (in AUTF8String key); */
                    Method {
                        name: "GetStringList",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIGSettingsService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
                    Method {
                        name: "GetCollectionForSchema",
                        params: &[Param { name: "schema", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIGSettingsCollection" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

