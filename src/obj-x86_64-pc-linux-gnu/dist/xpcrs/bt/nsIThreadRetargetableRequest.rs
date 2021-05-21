//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThreadRetargetableRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIThreadRetargetableRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void retargetDeliveryTo (in nsIEventTarget aNewTarget); */
                    Method {
                        name: "RetargetDeliveryTo",
                        params: &[Param { name: "aNewTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIEventTarget deliveryTarget; */
                    Method {
                        name: "GetDeliveryTarget",
                        params: &[Param { name: "aDeliveryTarget", ty: "*mut*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

