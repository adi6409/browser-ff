//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIReflowObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIReflowObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void reflow (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
                    Method {
                        name: "Reflow",
                        params: &[Param { name: "start", ty: "DOMHighResTimeStamp" }, Param { name: "end", ty: "DOMHighResTimeStamp" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void reflowInterruptible (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
                    Method {
                        name: "ReflowInterruptible",
                        params: &[Param { name: "start", ty: "DOMHighResTimeStamp" }, Param { name: "end", ty: "DOMHighResTimeStamp" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

