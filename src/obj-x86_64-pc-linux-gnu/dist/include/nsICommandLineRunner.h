/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineRunner.idl
 */

#ifndef __gen_nsICommandLineRunner_h__
#define __gen_nsICommandLineRunner_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsICommandLine_h__
#include "nsICommandLine.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICommandLineRunner */
#define NS_ICOMMANDLINERUNNER_IID_STR "c9f2996c-b25a-4d3d-821f-4cd0c4bc8afb"

#define NS_ICOMMANDLINERUNNER_IID \
  {0xc9f2996c, 0xb25a, 0x4d3d, \
    { 0x82, 0x1f, 0x4c, 0xd0, 0xc4, 0xbc, 0x8a, 0xfb }}

class NS_NO_VTABLE nsICommandLineRunner : public nsICommandLine {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMMANDLINERUNNER_IID)

  /* void init (in long argc, in nsArgvArray argv, in nsIFile workingDir, in unsigned long state); */
  NS_IMETHOD Init(int32_t argc, const char* const * argv, nsIFile *workingDir, uint32_t state) = 0;

  /* void run (); */
  NS_IMETHOD Run(void) = 0;

  /* readonly attribute AUTF8String helpText; */
  NS_IMETHOD GetHelpText(nsACString& aHelpText) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICommandLineRunner, NS_ICOMMANDLINERUNNER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMMANDLINERUNNER \
  NS_IMETHOD Init(int32_t argc, const char* const * argv, nsIFile *workingDir, uint32_t state) override; \
  NS_IMETHOD Run(void) override; \
  NS_IMETHOD GetHelpText(nsACString& aHelpText) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMMANDLINERUNNER \
  nsresult Init(int32_t argc, const char* const * argv, nsIFile *workingDir, uint32_t state); \
  nsresult Run(void); \
  nsresult GetHelpText(nsACString& aHelpText); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMMANDLINERUNNER(_to) \
  NS_IMETHOD Init(int32_t argc, const char* const * argv, nsIFile *workingDir, uint32_t state) override { return _to Init(argc, argv, workingDir, state); } \
  NS_IMETHOD Run(void) override { return _to Run(); } \
  NS_IMETHOD GetHelpText(nsACString& aHelpText) override { return _to GetHelpText(aHelpText); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMMANDLINERUNNER(_to) \
  NS_IMETHOD Init(int32_t argc, const char* const * argv, nsIFile *workingDir, uint32_t state) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(argc, argv, workingDir, state); } \
  NS_IMETHOD Run(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Run(); } \
  NS_IMETHOD GetHelpText(nsACString& aHelpText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHelpText(aHelpText); } 


#endif /* __gen_nsICommandLineRunner_h__ */
