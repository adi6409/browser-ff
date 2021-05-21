//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/geolocation/nsIDOMGeoPositionCoords.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDOMGeoPositionCoords",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute double latitude; */
                    Method {
                        name: "GetLatitude",
                        params: &[Param { name: "aLatitude", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double longitude; */
                    Method {
                        name: "GetLongitude",
                        params: &[Param { name: "aLongitude", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double altitude; */
                    Method {
                        name: "GetAltitude",
                        params: &[Param { name: "aAltitude", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double accuracy; */
                    Method {
                        name: "GetAccuracy",
                        params: &[Param { name: "aAccuracy", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double altitudeAccuracy; */
                    Method {
                        name: "GetAltitudeAccuracy",
                        params: &[Param { name: "aAltitudeAccuracy", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double heading; */
                    Method {
                        name: "GetHeading",
                        params: &[Param { name: "aHeading", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute double speed; */
                    Method {
                        name: "GetSpeed",
                        params: &[Param { name: "aSpeed", ty: "*mut libc::c_double" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

