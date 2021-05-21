/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPK11TokenDB.idl
 */

#ifndef __gen_nsIPK11TokenDB_h__
#define __gen_nsIPK11TokenDB_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPK11Token; /* forward declaration */

#define NS_PK11TOKENDB_CONTRACTID "@mozilla.org/security/pk11tokendb;1"

/* starting interface:    nsIPK11TokenDB */
#define NS_IPK11TOKENDB_IID_STR "4ee28c82-1dd2-11b2-aabf-bb4017abe395"

#define NS_IPK11TOKENDB_IID \
  {0x4ee28c82, 0x1dd2, 0x11b2, \
    { 0xaa, 0xbf, 0xbb, 0x40, 0x17, 0xab, 0xe3, 0x95 }}

class NS_NO_VTABLE nsIPK11TokenDB : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPK11TOKENDB_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPK11TokenDB;

  /* nsIPK11Token getInternalKeyToken (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInternalKeyToken(nsIPK11Token **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPK11TokenDB, NS_IPK11TOKENDB_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPK11TOKENDB \
  NS_IMETHOD GetInternalKeyToken(nsIPK11Token **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPK11TOKENDB \
  nsresult GetInternalKeyToken(nsIPK11Token **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPK11TOKENDB(_to) \
  NS_IMETHOD GetInternalKeyToken(nsIPK11Token **_retval) override { return _to GetInternalKeyToken(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPK11TOKENDB(_to) \
  NS_IMETHOD GetInternalKeyToken(nsIPK11Token **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInternalKeyToken(_retval); } 


#endif /* __gen_nsIPK11TokenDB_h__ */
