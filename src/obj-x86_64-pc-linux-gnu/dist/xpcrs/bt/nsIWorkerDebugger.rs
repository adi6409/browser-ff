//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebugger.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWorkerDebuggerListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onClose (); */
                    Method {
                        name: "OnClose",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void onError (in AString filename, in unsigned long lineno, in AString message); */
                    Method {
                        name: "OnError",
                        params: &[Param { name: "filename", ty: "*const ::nsstring::nsAString" }, Param { name: "lineno", ty: "u32" }, Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onMessage (in AString message); */
                    Method {
                        name: "OnMessage",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWorkerDebugger",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute bool isClosed; */
                    Method {
                        name: "GetIsClosed",
                        params: &[Param { name: "aIsClosed", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isChrome; */
                    Method {
                        name: "GetIsChrome",
                        params: &[Param { name: "aIsChrome", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool isInitialized; */
                    Method {
                        name: "GetIsInitialized",
                        params: &[Param { name: "aIsInitialized", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIWorkerDebugger parent; */
                    Method {
                        name: "GetParent",
                        params: &[Param { name: "aParent", ty: "*mut *const nsIWorkerDebugger" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString url; */
                    Method {
                        name: "GetUrl",
                        params: &[Param { name: "aUrl", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindow window; */
                    Method {
                        name: "GetWindow",
                        params: &[Param { name: "aWindow", ty: "*mut*const mozIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<uint64_t> windowIDs; */
                    Method {
                        name: "GetWindowIDs",
                        params: &[Param { name: "aWindowIDs", ty: "*mut thin_vec::ThinVec<uint64_t>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long serviceWorkerID; */
                    Method {
                        name: "GetServiceWorkerID",
                        params: &[Param { name: "aServiceWorkerID", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString id; */
                    Method {
                        name: "GetId",
                        params: &[Param { name: "aId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initialize (in AString url); */
                    Method {
                        name: "Initialize",
                        params: &[Param { name: "url", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(PostMessageMoz)] void postMessage (in AString message); */
                    Method {
                        name: "PostMessageMoz",
                        params: &[Param { name: "message", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addListener (in nsIWorkerDebuggerListener listener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeListener (in nsIWorkerDebuggerListener listener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "listener", ty: "*const nsIWorkerDebuggerListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDebuggerReady (in boolean ready); */
                    Method {
                        name: "SetDebuggerReady",
                        params: &[Param { name: "ready", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

