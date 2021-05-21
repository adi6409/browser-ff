//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthenticableChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpAuthenticableChannel",
            base: Some("nsIProxiedChannel"),
            methods: Ok(&[
                    /* [must_use] readonly attribute boolean isSSL; */
                    Method {
                        name: "GetIsSSL",
                        params: &[Param { name: "aIsSSL", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean proxyMethodIsConnect; */
                    Method {
                        name: "GetProxyMethodIsConnect",
                        params: &[Param { name: "aProxyMethodIsConnect", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void cancel (in nsresult aStatus); */
                    Method {
                        name: "Cancel",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsLoadFlags loadFlags; */
                    Method {
                        name: "GetLoadFlags",
                        params: &[Param { name: "aLoadFlags", ty: "*mut nsLoadFlags" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "GetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute nsIInterfaceRequestor notificationCallbacks; */
                    Method {
                        name: "GetNotificationCallbacks",
                        params: &[Param { name: "aNotificationCallbacks", ty: "*mut*const nsIInterfaceRequestor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString requestMethod; */
                    Method {
                        name: "GetRequestMethod",
                        params: &[Param { name: "aRequestMethod", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString serverResponseHeader; */
                    Method {
                        name: "GetServerResponseHeader",
                        params: &[Param { name: "aServerResponseHeader", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString proxyChallenges; */
                    Method {
                        name: "GetProxyChallenges",
                        params: &[Param { name: "aProxyChallenges", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString WWWChallenges; */
                    Method {
                        name: "GetWWWChallenges",
                        params: &[Param { name: "aWWWChallenges", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setProxyCredentials (in ACString credentials); */
                    Method {
                        name: "SetProxyCredentials",
                        params: &[Param { name: "credentials", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setWWWCredentials (in ACString credentials); */
                    Method {
                        name: "SetWWWCredentials",
                        params: &[Param { name: "credentials", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onAuthAvailable (); */
                    Method {
                        name: "OnAuthAvailable",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void onAuthCancelled (in boolean userCancel); */
                    Method {
                        name: "OnAuthCancelled",
                        params: &[Param { name: "userCancel", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void closeStickyConnection (); */
                    Method {
                        name: "CloseStickyConnection",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void connectionRestartable (in boolean restartable); */
                    Method {
                        name: "ConnectionRestartable",
                        params: &[Param { name: "restartable", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

