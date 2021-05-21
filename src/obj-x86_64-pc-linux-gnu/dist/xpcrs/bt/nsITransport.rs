//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITransport.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransport",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIInputStream openInputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
                    Method {
                        name: "OpenInputStream",
                        params: &[Param { name: "aFlags", ty: "u32" }, Param { name: "aSegmentSize", ty: "u32" }, Param { name: "aSegmentCount", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIOutputStream openOutputStream (in unsigned long aFlags, in unsigned long aSegmentSize, in unsigned long aSegmentCount); */
                    Method {
                        name: "OpenOutputStream",
                        params: &[Param { name: "aFlags", ty: "u32" }, Param { name: "aSegmentSize", ty: "u32" }, Param { name: "aSegmentCount", ty: "u32" }, Param { name: "_retval", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (in nsresult aReason); */
                    Method {
                        name: "Close",
                        params: &[Param { name: "aReason", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setEventSink (in nsITransportEventSink aSink, in nsIEventTarget aEventTarget); */
                    Method {
                        name: "SetEventSink",
                        params: &[Param { name: "aSink", ty: "*const nsITransportEventSink" }, Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITransportEventSink",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onTransportStatus (in nsITransport aTransport, in nsresult aStatus, in long long aProgress, in long long aProgressMax); */
                    Method {
                        name: "OnTransportStatus",
                        params: &[Param { name: "aTransport", ty: "*const nsITransport" }, Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aProgress", ty: "i64" }, Param { name: "aProgressMax", ty: "i64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

