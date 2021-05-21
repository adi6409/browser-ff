/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIPlacesAutoComplete.idl
 */

#ifndef __gen_mozIPlacesAutoComplete_h__
#define __gen_mozIPlacesAutoComplete_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    mozIPlacesAutoComplete */
#define MOZIPLACESAUTOCOMPLETE_IID_STR "61b6348a-09e1-4810-8057-f8cb3cec6ef8"

#define MOZIPLACESAUTOCOMPLETE_IID \
  {0x61b6348a, 0x09e1, 0x4810, \
    { 0x80, 0x57, 0xf8, 0xcb, 0x3c, 0xec, 0x6e, 0xf8 }}

class NS_NO_VTABLE mozIPlacesAutoComplete : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIPLACESAUTOCOMPLETE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIPlacesAutoComplete;

  enum {
    MATCH_ANYWHERE = 0,
    MATCH_BOUNDARY_ANYWHERE = 1,
    MATCH_BOUNDARY = 2,
    MATCH_BEGINNING = 3,
    MATCH_ANYWHERE_UNMODIFIED = 4,
    MATCH_BEGINNING_CASE_SENSITIVE = 5,
    BEHAVIOR_HISTORY = 1,
    BEHAVIOR_BOOKMARK = 2,
    BEHAVIOR_TAG = 4,
    BEHAVIOR_TITLE = 8,
    BEHAVIOR_URL = 16,
    BEHAVIOR_TYPED = 32,
    BEHAVIOR_JAVASCRIPT = 64,
    BEHAVIOR_OPENPAGE = 128,
    BEHAVIOR_RESTRICT = 256,
    BEHAVIOR_SEARCH = 512
  };

  /* void populatePreloadedSiteStorage (in jsval sites); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PopulatePreloadedSiteStorage(JS::HandleValue sites) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIPlacesAutoComplete, MOZIPLACESAUTOCOMPLETE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIPLACESAUTOCOMPLETE \
  NS_IMETHOD PopulatePreloadedSiteStorage(JS::HandleValue sites) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIPLACESAUTOCOMPLETE \
  nsresult PopulatePreloadedSiteStorage(JS::HandleValue sites); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIPLACESAUTOCOMPLETE(_to) \
  NS_IMETHOD PopulatePreloadedSiteStorage(JS::HandleValue sites) override { return _to PopulatePreloadedSiteStorage(sites); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIPLACESAUTOCOMPLETE(_to) \
  NS_IMETHOD PopulatePreloadedSiteStorage(JS::HandleValue sites) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PopulatePreloadedSiteStorage(sites); } 


#endif /* __gen_mozIPlacesAutoComplete_h__ */
