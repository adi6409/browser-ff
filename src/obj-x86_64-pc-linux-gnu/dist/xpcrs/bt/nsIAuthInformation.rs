//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthInformation.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAuthInformation",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long flags; */
                    Method {
                        name: "GetFlags",
                        params: &[Param { name: "aFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString realm; */
                    Method {
                        name: "GetRealm",
                        params: &[Param { name: "aRealm", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String authenticationScheme; */
                    Method {
                        name: "GetAuthenticationScheme",
                        params: &[Param { name: "aAuthenticationScheme", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString username; */
                    Method {
                        name: "GetUsername",
                        params: &[Param { name: "aUsername", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetUsername",
                        params: &[Param { name: "aUsername", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString password; */
                    Method {
                        name: "GetPassword",
                        params: &[Param { name: "aPassword", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPassword",
                        params: &[Param { name: "aPassword", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString domain; */
                    Method {
                        name: "GetDomain",
                        params: &[Param { name: "aDomain", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDomain",
                        params: &[Param { name: "aDomain", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

