//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsITaskbarProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITaskbarProgress",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setProgressState (in nsTaskbarProgressState state, [optional] in unsigned long long currentValue, [optional] in unsigned long long maxValue); */
                    Method {
                        name: "SetProgressState",
                        params: &[Param { name: "state", ty: "nsTaskbarProgressState" }, Param { name: "currentValue", ty: "u64" }, Param { name: "maxValue", ty: "u64" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

