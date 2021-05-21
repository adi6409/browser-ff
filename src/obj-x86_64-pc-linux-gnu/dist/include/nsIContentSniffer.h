/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIContentSniffer.idl
 */

#ifndef __gen_nsIContentSniffer_h__
#define __gen_nsIContentSniffer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRequest; /* forward declaration */


/* starting interface:    nsIContentSniffer */
#define NS_ICONTENTSNIFFER_IID_STR "a5772d1b-fc63-495e-a169-96e8d3311af0"

#define NS_ICONTENTSNIFFER_IID \
  {0xa5772d1b, 0xfc63, 0x495e, \
    { 0xa1, 0x69, 0x96, 0xe8, 0xd3, 0x31, 0x1a, 0xf0 }}

class NS_NO_VTABLE nsIContentSniffer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONTENTSNIFFER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIContentSniffer;

  /* ACString getMIMETypeFromContent (in nsIRequest aRequest, [array, size_is (aLength), const] in octet aData, in unsigned long aLength); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMIMETypeFromContent(nsIRequest *aRequest, const uint8_t *aData, uint32_t aLength, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIContentSniffer, NS_ICONTENTSNIFFER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONTENTSNIFFER \
  NS_IMETHOD GetMIMETypeFromContent(nsIRequest *aRequest, const uint8_t *aData, uint32_t aLength, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONTENTSNIFFER \
  nsresult GetMIMETypeFromContent(nsIRequest *aRequest, const uint8_t *aData, uint32_t aLength, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONTENTSNIFFER(_to) \
  NS_IMETHOD GetMIMETypeFromContent(nsIRequest *aRequest, const uint8_t *aData, uint32_t aLength, nsACString& _retval) override { return _to GetMIMETypeFromContent(aRequest, aData, aLength, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONTENTSNIFFER(_to) \
  NS_IMETHOD GetMIMETypeFromContent(nsIRequest *aRequest, const uint8_t *aData, uint32_t aLength, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMIMETypeFromContent(aRequest, aData, aLength, _retval); } 


#endif /* __gen_nsIContentSniffer_h__ */
