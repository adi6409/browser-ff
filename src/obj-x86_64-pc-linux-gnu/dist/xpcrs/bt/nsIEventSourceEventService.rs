//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIEventSourceEventService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIEventSourceEventListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void eventSourceConnectionOpened (in uint64_t aHttpChannelId); */
                    Method {
                        name: "EventSourceConnectionOpened",
                        params: &[Param { name: "aHttpChannelId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void eventSourceConnectionClosed (in uint64_t aHttpChannelId); */
                    Method {
                        name: "EventSourceConnectionClosed",
                        params: &[Param { name: "aHttpChannelId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void eventReceived (in uint64_t aHttpChannelId, in AString aEventName, in AString aLastEventID, in AString aData, in uint32_t aRetry, in DOMHighResTimeStamp aTimeStamp); */
                    Method {
                        name: "EventReceived",
                        params: &[Param { name: "aHttpChannelId", ty: "uint64_t" }, Param { name: "aEventName", ty: "*const ::nsstring::nsAString" }, Param { name: "aLastEventID", ty: "*const ::nsstring::nsAString" }, Param { name: "aData", ty: "*const ::nsstring::nsAString" }, Param { name: "aRetry", ty: "uint32_t" }, Param { name: "aTimeStamp", ty: "DOMHighResTimeStamp" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIEventSourceEventService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] void addListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aListener", ty: "*const nsIEventSourceEventListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void removeListener (in unsigned long long aInnerWindowID, in nsIEventSourceEventListener aListener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "aListener", ty: "*const nsIEventSourceEventListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] bool hasListenerFor (in unsigned long long aInnerWindowID); */
                    Method {
                        name: "HasListenerFor",
                        params: &[Param { name: "aInnerWindowID", ty: "u64" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

