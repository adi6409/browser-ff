//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsITimer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITimerCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notify (in nsITimer timer); */
                    Method {
                        name: "Notify",
                        params: &[Param { name: "timer", ty: "*const nsITimer" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITimer",
            base: Some("nsISupports"),
            methods: Err("native type mozilla::TimeDuration unsupported"),
        },

        ]; D}

