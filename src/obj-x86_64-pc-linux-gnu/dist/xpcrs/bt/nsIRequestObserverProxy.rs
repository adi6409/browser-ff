//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestObserverProxy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRequestObserverProxy",
            base: Some("nsIRequestObserver"),
            methods: Ok(&[
                    /* void init (in nsIRequestObserver observer, in nsISupports context); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "observer", ty: "*const nsIRequestObserver" }, Param { name: "context", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

