/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineValidator.idl
 */

#ifndef __gen_nsICommandLineValidator_h__
#define __gen_nsICommandLineValidator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsICommandLine; /* forward declaration */


/* starting interface:    nsICommandLineValidator */
#define NS_ICOMMANDLINEVALIDATOR_IID_STR "5ecaa593-7660-4a3a-957a-92d5770671c7"

#define NS_ICOMMANDLINEVALIDATOR_IID \
  {0x5ecaa593, 0x7660, 0x4a3a, \
    { 0x95, 0x7a, 0x92, 0xd5, 0x77, 0x06, 0x71, 0xc7 }}

class NS_NO_VTABLE nsICommandLineValidator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDLINEVALIDATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICommandLineValidator;

  /* void validate (in nsICommandLine aCommandLine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Validate(nsICommandLine *aCommandLine) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandLineValidator, NS_ICOMMANDLINEVALIDATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDLINEVALIDATOR \
  NS_IMETHOD Validate(nsICommandLine *aCommandLine) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDLINEVALIDATOR \
  nsresult Validate(nsICommandLine *aCommandLine); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDLINEVALIDATOR(_to) \
  NS_IMETHOD Validate(nsICommandLine *aCommandLine) override { return _to Validate(aCommandLine); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDLINEVALIDATOR(_to) \
  NS_IMETHOD Validate(nsICommandLine *aCommandLine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Validate(aCommandLine); } 


#endif /* __gen_nsICommandLineValidator_h__ */
