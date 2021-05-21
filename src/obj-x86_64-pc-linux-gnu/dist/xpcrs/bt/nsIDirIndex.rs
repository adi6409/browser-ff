//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndex.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDirIndex",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute unsigned long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetType",
                        params: &[Param { name: "aType", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString contentType; */
                    Method {
                        name: "GetContentType",
                        params: &[Param { name: "aContentType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetContentType",
                        params: &[Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute ACString location; */
                    Method {
                        name: "GetLocation",
                        params: &[Param { name: "aLocation", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLocation",
                        params: &[Param { name: "aLocation", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDescription",
                        params: &[Param { name: "aDescription", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long long size; */
                    Method {
                        name: "GetSize",
                        params: &[Param { name: "aSize", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSize",
                        params: &[Param { name: "aSize", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute PRTime lastModified; */
                    Method {
                        name: "GetLastModified",
                        params: &[Param { name: "aLastModified", ty: "*mut PRTime" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLastModified",
                        params: &[Param { name: "aLastModified", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

