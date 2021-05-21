//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11Token.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPK11Token",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] readonly attribute AUTF8String tokenName; */
                    Method {
                        name: "GetTokenName",
                        params: &[Param { name: "aTokenName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean isInternalKeyToken; */
                    Method {
                        name: "GetIsInternalKeyToken",
                        params: &[Param { name: "aIsInternalKeyToken", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String tokenManID; */
                    Method {
                        name: "GetTokenManID",
                        params: &[Param { name: "aTokenManID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String tokenHWVersion; */
                    Method {
                        name: "GetTokenHWVersion",
                        params: &[Param { name: "aTokenHWVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String tokenFWVersion; */
                    Method {
                        name: "GetTokenFWVersion",
                        params: &[Param { name: "aTokenFWVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute AUTF8String tokenSerialNumber; */
                    Method {
                        name: "GetTokenSerialNumber",
                        params: &[Param { name: "aTokenSerialNumber", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isLoggedIn (); */
                    Method {
                        name: "IsLoggedIn",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void login (in boolean force); */
                    Method {
                        name: "Login",
                        params: &[Param { name: "force", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void logoutSimple (); */
                    Method {
                        name: "LogoutSimple",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void logoutAndDropAuthenticatedResources (); */
                    Method {
                        name: "LogoutAndDropAuthenticatedResources",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean needsLogin (); */
                    Method {
                        name: "NeedsLogin",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean needsUserInit; */
                    Method {
                        name: "GetNeedsUserInit",
                        params: &[Param { name: "aNeedsUserInit", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void reset (); */
                    Method {
                        name: "Reset",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean checkPassword (in AUTF8String password); */
                    Method {
                        name: "CheckPassword",
                        params: &[Param { name: "password", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void initPassword (in AUTF8String initialPassword); */
                    Method {
                        name: "InitPassword",
                        params: &[Param { name: "initialPassword", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void changePassword (in AUTF8String oldPassword, in AUTF8String newPassword); */
                    Method {
                        name: "ChangePassword",
                        params: &[Param { name: "oldPassword", ty: "*const ::nsstring::nsACString" }, Param { name: "newPassword", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean hasPassword; */
                    Method {
                        name: "GetHasPassword",
                        params: &[Param { name: "aHasPassword", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

