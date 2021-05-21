//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/timermanager/nsIUpdateTimerManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUpdateTimerManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerTimer (in AString id, in nsITimerCallback callback, in unsigned long interval, [optional] in boolean skipFirst); */
                    Method {
                        name: "RegisterTimer",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsAString" }, Param { name: "callback", ty: "*const nsITimerCallback" }, Param { name: "interval", ty: "u32" }, Param { name: "skipFirst", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterTimer (in AString id); */
                    Method {
                        name: "UnregisterTimer",
                        params: &[Param { name: "id", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

