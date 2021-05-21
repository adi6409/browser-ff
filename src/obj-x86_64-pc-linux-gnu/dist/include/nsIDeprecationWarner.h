/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDeprecationWarner.idl
 */

#ifndef __gen_nsIDeprecationWarner_h__
#define __gen_nsIDeprecationWarner_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDeprecationWarner */
#define NS_IDEPRECATIONWARNER_IID_STR "665c5124-2c52-41ba-ae72-2393f8e76c25"

#define NS_IDEPRECATIONWARNER_IID \
  {0x665c5124, 0x2c52, 0x41ba, \
    { 0xae, 0x72, 0x23, 0x93, 0xf8, 0xe7, 0x6c, 0x25 }}

class NS_NO_VTABLE nsIDeprecationWarner : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDEPRECATIONWARNER_IID)

  /* void issueWarning (in uint32_t aWarning, [optional] in bool aAsError); */
  NS_IMETHOD IssueWarning(uint32_t aWarning, bool aAsError) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDeprecationWarner, NS_IDEPRECATIONWARNER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDEPRECATIONWARNER \
  NS_IMETHOD IssueWarning(uint32_t aWarning, bool aAsError) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDEPRECATIONWARNER \
  nsresult IssueWarning(uint32_t aWarning, bool aAsError); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDEPRECATIONWARNER(_to) \
  NS_IMETHOD IssueWarning(uint32_t aWarning, bool aAsError) override { return _to IssueWarning(aWarning, aAsError); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDEPRECATIONWARNER(_to) \
  NS_IMETHOD IssueWarning(uint32_t aWarning, bool aAsError) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IssueWarning(aWarning, aAsError); } 


#endif /* __gen_nsIDeprecationWarner_h__ */
