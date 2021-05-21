//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListenerTee.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStreamListenerTee",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* void init (in nsIStreamListener listener, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "listener", ty: "*const nsIStreamListener" }, Param { name: "sink", ty: "*const nsIOutputStream" }, Param { name: "requestObserver", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initAsync (in nsIStreamListener listener, in nsIEventTarget eventTarget, in nsIOutputStream sink, [optional] in nsIRequestObserver requestObserver); */
                    Method {
                        name: "InitAsync",
                        params: &[Param { name: "listener", ty: "*const nsIStreamListener" }, Param { name: "eventTarget", ty: "*const nsIEventTarget" }, Param { name: "sink", ty: "*const nsIOutputStream" }, Param { name: "requestObserver", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

