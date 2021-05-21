//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsILoginInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString displayOrigin; */
                    Method {
                        name: "GetDisplayOrigin",
                        params: &[Param { name: "aDisplayOrigin", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString origin; */
                    Method {
                        name: "GetOrigin",
                        params: &[Param { name: "aOrigin", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetOrigin",
                        params: &[Param { name: "aOrigin", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString hostname; */
                    Method {
                        name: "GetHostname",
                        params: &[Param { name: "aHostname", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetHostname",
                        params: &[Param { name: "aHostname", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString formActionOrigin; */
                    Method {
                        name: "GetFormActionOrigin",
                        params: &[Param { name: "aFormActionOrigin", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFormActionOrigin",
                        params: &[Param { name: "aFormActionOrigin", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString formSubmitURL; */
                    Method {
                        name: "GetFormSubmitURL",
                        params: &[Param { name: "aFormSubmitURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetFormSubmitURL",
                        params: &[Param { name: "aFormSubmitURL", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute AString httpRealm; */
                    Method {
                        name: "GetHttpRealm",
                        params: &[Param { name: "aHttpRealm", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetHttpRealm",
                        params: &[Param { name: "aHttpRealm", ty: "*const ::nsstring::nsAString" }],
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

                    /* attribute AString usernameField; */
                    Method {
                        name: "GetUsernameField",
                        params: &[Param { name: "aUsernameField", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetUsernameField",
                        params: &[Param { name: "aUsernameField", ty: "*const ::nsstring::nsAString" }],
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

                    /* attribute AString passwordField; */
                    Method {
                        name: "GetPasswordField",
                        params: &[Param { name: "aPasswordField", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPasswordField",
                        params: &[Param { name: "aPasswordField", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in AString aOrigin, in AString aFormActionOrigin, in AString aHttpRealm, in AString aUsername, in AString aPassword, [optional] in AString aUsernameField, [optional] in AString aPasswordField); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aOrigin", ty: "*const ::nsstring::nsAString" }, Param { name: "aFormActionOrigin", ty: "*const ::nsstring::nsAString" }, Param { name: "aHttpRealm", ty: "*const ::nsstring::nsAString" }, Param { name: "aUsername", ty: "*const ::nsstring::nsAString" }, Param { name: "aPassword", ty: "*const ::nsstring::nsAString" }, Param { name: "aUsernameField", ty: "*const ::nsstring::nsAString" }, Param { name: "aPasswordField", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean equals (in nsILoginInfo aLoginInfo); */
                    Method {
                        name: "Equals",
                        params: &[Param { name: "aLoginInfo", ty: "*const nsILoginInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
                    Method {
                        name: "Matches",
                        params: &[Param { name: "aLoginInfo", ty: "*const nsILoginInfo" }, Param { name: "ignorePassword", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsILoginInfo clone (); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "_retval", ty: "*mut *const nsILoginInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

