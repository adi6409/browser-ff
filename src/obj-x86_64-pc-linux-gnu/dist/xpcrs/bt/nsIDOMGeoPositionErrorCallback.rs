//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionErrorCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPositionErrorCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleEvent (in GeolocationPositionError positionError); */
                    Method {
                        name: "HandleEvent",
                        params: &[Param { name: "positionError", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

