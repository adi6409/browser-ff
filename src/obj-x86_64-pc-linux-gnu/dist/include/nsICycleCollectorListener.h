/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsICycleCollectorListener.idl
 */

#ifndef __gen_nsICycleCollectorListener_h__
#define __gen_nsICycleCollectorListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include <stdio.h>
class nsCycleCollectorLogger;
class nsIFile; /* forward declaration */


/* starting interface:    nsICycleCollectorHandler */
#define NS_ICYCLECOLLECTORHANDLER_IID_STR "7f093367-1492-4b89-87af-c01dbc831246"

#define NS_ICYCLECOLLECTORHANDLER_IID \
  {0x7f093367, 0x1492, 0x4b89, \
    { 0x87, 0xaf, 0xc0, 0x1d, 0xbc, 0x83, 0x12, 0x46 }}

class NS_NO_VTABLE nsICycleCollectorHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICYCLECOLLECTORHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICycleCollectorHandler;

  /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NoteRefCountedObject(const nsACString& aAddress, uint32_t aRefCount, const nsACString& aObjectDescription) = 0;

  /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NoteGCedObject(const nsACString& aAddress, bool aMarked, const nsACString& aObjectDescription, const nsACString& aCompartmentAddress) = 0;

  /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NoteEdge(const nsACString& aFromAddress, const nsACString& aToAddress, const nsACString& aEdgeName) = 0;

  /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DescribeRoot(const nsACString& aAddress, uint32_t aKnownEdges) = 0;

  /* void describeGarbage (in ACString aAddress); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DescribeGarbage(const nsACString& aAddress) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICycleCollectorHandler, NS_ICYCLECOLLECTORHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICYCLECOLLECTORHANDLER \
  NS_IMETHOD NoteRefCountedObject(const nsACString& aAddress, uint32_t aRefCount, const nsACString& aObjectDescription) override; \
  NS_IMETHOD NoteGCedObject(const nsACString& aAddress, bool aMarked, const nsACString& aObjectDescription, const nsACString& aCompartmentAddress) override; \
  NS_IMETHOD NoteEdge(const nsACString& aFromAddress, const nsACString& aToAddress, const nsACString& aEdgeName) override; \
  NS_IMETHOD DescribeRoot(const nsACString& aAddress, uint32_t aKnownEdges) override; \
  NS_IMETHOD DescribeGarbage(const nsACString& aAddress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICYCLECOLLECTORHANDLER \
  nsresult NoteRefCountedObject(const nsACString& aAddress, uint32_t aRefCount, const nsACString& aObjectDescription); \
  nsresult NoteGCedObject(const nsACString& aAddress, bool aMarked, const nsACString& aObjectDescription, const nsACString& aCompartmentAddress); \
  nsresult NoteEdge(const nsACString& aFromAddress, const nsACString& aToAddress, const nsACString& aEdgeName); \
  nsresult DescribeRoot(const nsACString& aAddress, uint32_t aKnownEdges); \
  nsresult DescribeGarbage(const nsACString& aAddress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICYCLECOLLECTORHANDLER(_to) \
  NS_IMETHOD NoteRefCountedObject(const nsACString& aAddress, uint32_t aRefCount, const nsACString& aObjectDescription) override { return _to NoteRefCountedObject(aAddress, aRefCount, aObjectDescription); } \
  NS_IMETHOD NoteGCedObject(const nsACString& aAddress, bool aMarked, const nsACString& aObjectDescription, const nsACString& aCompartmentAddress) override { return _to NoteGCedObject(aAddress, aMarked, aObjectDescription, aCompartmentAddress); } \
  NS_IMETHOD NoteEdge(const nsACString& aFromAddress, const nsACString& aToAddress, const nsACString& aEdgeName) override { return _to NoteEdge(aFromAddress, aToAddress, aEdgeName); } \
  NS_IMETHOD DescribeRoot(const nsACString& aAddress, uint32_t aKnownEdges) override { return _to DescribeRoot(aAddress, aKnownEdges); } \
  NS_IMETHOD DescribeGarbage(const nsACString& aAddress) override { return _to DescribeGarbage(aAddress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICYCLECOLLECTORHANDLER(_to) \
  NS_IMETHOD NoteRefCountedObject(const nsACString& aAddress, uint32_t aRefCount, const nsACString& aObjectDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NoteRefCountedObject(aAddress, aRefCount, aObjectDescription); } \
  NS_IMETHOD NoteGCedObject(const nsACString& aAddress, bool aMarked, const nsACString& aObjectDescription, const nsACString& aCompartmentAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NoteGCedObject(aAddress, aMarked, aObjectDescription, aCompartmentAddress); } \
  NS_IMETHOD NoteEdge(const nsACString& aFromAddress, const nsACString& aToAddress, const nsACString& aEdgeName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NoteEdge(aFromAddress, aToAddress, aEdgeName); } \
  NS_IMETHOD DescribeRoot(const nsACString& aAddress, uint32_t aKnownEdges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DescribeRoot(aAddress, aKnownEdges); } \
  NS_IMETHOD DescribeGarbage(const nsACString& aAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DescribeGarbage(aAddress); } 


/* starting interface:    nsICycleCollectorLogSink */
#define NS_ICYCLECOLLECTORLOGSINK_IID_STR "3ad9875f-d0e4-4ac2-87e3-f127f6c02ce1"

#define NS_ICYCLECOLLECTORLOGSINK_IID \
  {0x3ad9875f, 0xd0e4, 0x4ac2, \
    { 0x87, 0xe3, 0xf1, 0x27, 0xf6, 0xc0, 0x2c, 0xe1 }}

class NS_NO_VTABLE nsICycleCollectorLogSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICYCLECOLLECTORLOGSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICycleCollectorLogSink;

  /* [noscript] void open (out FILE aGCLog, out FILE aCCLog); */
  NS_IMETHOD Open(FILE * * aGCLog, FILE * * aCCLog) = 0;

  /* void closeGCLog (); */
  NS_IMETHOD CloseGCLog(void) = 0;

  /* void closeCCLog (); */
  NS_IMETHOD CloseCCLog(void) = 0;

  /* attribute AString filenameIdentifier; */
  NS_IMETHOD GetFilenameIdentifier(nsAString& aFilenameIdentifier) = 0;
  NS_IMETHOD SetFilenameIdentifier(const nsAString& aFilenameIdentifier) = 0;

  /* attribute int32_t processIdentifier; */
  NS_IMETHOD GetProcessIdentifier(int32_t *aProcessIdentifier) = 0;
  NS_IMETHOD SetProcessIdentifier(int32_t aProcessIdentifier) = 0;

  /* readonly attribute nsIFile gcLog; */
  NS_IMETHOD GetGcLog(nsIFile **aGcLog) = 0;

  /* readonly attribute nsIFile ccLog; */
  NS_IMETHOD GetCcLog(nsIFile **aCcLog) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICycleCollectorLogSink, NS_ICYCLECOLLECTORLOGSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICYCLECOLLECTORLOGSINK \
  NS_IMETHOD Open(FILE * * aGCLog, FILE * * aCCLog) override; \
  NS_IMETHOD CloseGCLog(void) override; \
  NS_IMETHOD CloseCCLog(void) override; \
  NS_IMETHOD GetFilenameIdentifier(nsAString& aFilenameIdentifier) override; \
  NS_IMETHOD SetFilenameIdentifier(const nsAString& aFilenameIdentifier) override; \
  NS_IMETHOD GetProcessIdentifier(int32_t *aProcessIdentifier) override; \
  NS_IMETHOD SetProcessIdentifier(int32_t aProcessIdentifier) override; \
  NS_IMETHOD GetGcLog(nsIFile **aGcLog) override; \
  NS_IMETHOD GetCcLog(nsIFile **aCcLog) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICYCLECOLLECTORLOGSINK \
  nsresult Open(FILE * * aGCLog, FILE * * aCCLog); \
  nsresult CloseGCLog(void); \
  nsresult CloseCCLog(void); \
  nsresult GetFilenameIdentifier(nsAString& aFilenameIdentifier); \
  nsresult SetFilenameIdentifier(const nsAString& aFilenameIdentifier); \
  nsresult GetProcessIdentifier(int32_t *aProcessIdentifier); \
  nsresult SetProcessIdentifier(int32_t aProcessIdentifier); \
  nsresult GetGcLog(nsIFile **aGcLog); \
  nsresult GetCcLog(nsIFile **aCcLog); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICYCLECOLLECTORLOGSINK(_to) \
  NS_IMETHOD Open(FILE * * aGCLog, FILE * * aCCLog) override { return _to Open(aGCLog, aCCLog); } \
  NS_IMETHOD CloseGCLog(void) override { return _to CloseGCLog(); } \
  NS_IMETHOD CloseCCLog(void) override { return _to CloseCCLog(); } \
  NS_IMETHOD GetFilenameIdentifier(nsAString& aFilenameIdentifier) override { return _to GetFilenameIdentifier(aFilenameIdentifier); } \
  NS_IMETHOD SetFilenameIdentifier(const nsAString& aFilenameIdentifier) override { return _to SetFilenameIdentifier(aFilenameIdentifier); } \
  NS_IMETHOD GetProcessIdentifier(int32_t *aProcessIdentifier) override { return _to GetProcessIdentifier(aProcessIdentifier); } \
  NS_IMETHOD SetProcessIdentifier(int32_t aProcessIdentifier) override { return _to SetProcessIdentifier(aProcessIdentifier); } \
  NS_IMETHOD GetGcLog(nsIFile **aGcLog) override { return _to GetGcLog(aGcLog); } \
  NS_IMETHOD GetCcLog(nsIFile **aCcLog) override { return _to GetCcLog(aCcLog); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICYCLECOLLECTORLOGSINK(_to) \
  NS_IMETHOD Open(FILE * * aGCLog, FILE * * aCCLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aGCLog, aCCLog); } \
  NS_IMETHOD CloseGCLog(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseGCLog(); } \
  NS_IMETHOD CloseCCLog(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseCCLog(); } \
  NS_IMETHOD GetFilenameIdentifier(nsAString& aFilenameIdentifier) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilenameIdentifier(aFilenameIdentifier); } \
  NS_IMETHOD SetFilenameIdentifier(const nsAString& aFilenameIdentifier) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFilenameIdentifier(aFilenameIdentifier); } \
  NS_IMETHOD GetProcessIdentifier(int32_t *aProcessIdentifier) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessIdentifier(aProcessIdentifier); } \
  NS_IMETHOD SetProcessIdentifier(int32_t aProcessIdentifier) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProcessIdentifier(aProcessIdentifier); } \
  NS_IMETHOD GetGcLog(nsIFile **aGcLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGcLog(aGcLog); } \
  NS_IMETHOD GetCcLog(nsIFile **aCcLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCcLog(aCcLog); } 


/* starting interface:    nsICycleCollectorListener */
#define NS_ICYCLECOLLECTORLISTENER_IID_STR "703b53b6-24f6-40c6-9ea9-aeb2dc53d170"

#define NS_ICYCLECOLLECTORLISTENER_IID \
  {0x703b53b6, 0x24f6, 0x40c6, \
    { 0x9e, 0xa9, 0xae, 0xb2, 0xdc, 0x53, 0xd1, 0x70 }}

class NS_NO_VTABLE nsICycleCollectorListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICYCLECOLLECTORLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICycleCollectorListener;

  /* nsICycleCollectorListener allTraces (); */
  NS_IMETHOD AllTraces(nsICycleCollectorListener **_retval) = 0;

  /* readonly attribute boolean wantAllTraces; */
  NS_IMETHOD GetWantAllTraces(bool *aWantAllTraces) = 0;

  /* attribute boolean disableLog; */
  NS_IMETHOD GetDisableLog(bool *aDisableLog) = 0;
  NS_IMETHOD SetDisableLog(bool aDisableLog) = 0;

  /* attribute nsICycleCollectorLogSink logSink; */
  NS_IMETHOD GetLogSink(nsICycleCollectorLogSink **aLogSink) = 0;
  NS_IMETHOD SetLogSink(nsICycleCollectorLogSink *aLogSink) = 0;

  /* attribute boolean wantAfterProcessing; */
  NS_IMETHOD GetWantAfterProcessing(bool *aWantAfterProcessing) = 0;
  NS_IMETHOD SetWantAfterProcessing(bool aWantAfterProcessing) = 0;

  /* boolean processNext (in nsICycleCollectorHandler aHandler); */
  NS_IMETHOD ProcessNext(nsICycleCollectorHandler *aHandler, bool *_retval) = 0;

  /* [noscript] nsCycleCollectorLoggerPtr asLogger (); */
  NS_IMETHOD AsLogger(nsCycleCollectorLogger * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICycleCollectorListener, NS_ICYCLECOLLECTORLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICYCLECOLLECTORLISTENER \
  NS_IMETHOD AllTraces(nsICycleCollectorListener **_retval) override; \
  NS_IMETHOD GetWantAllTraces(bool *aWantAllTraces) override; \
  NS_IMETHOD GetDisableLog(bool *aDisableLog) override; \
  NS_IMETHOD SetDisableLog(bool aDisableLog) override; \
  NS_IMETHOD GetLogSink(nsICycleCollectorLogSink **aLogSink) override; \
  NS_IMETHOD SetLogSink(nsICycleCollectorLogSink *aLogSink) override; \
  NS_IMETHOD GetWantAfterProcessing(bool *aWantAfterProcessing) override; \
  NS_IMETHOD SetWantAfterProcessing(bool aWantAfterProcessing) override; \
  NS_IMETHOD ProcessNext(nsICycleCollectorHandler *aHandler, bool *_retval) override; \
  NS_IMETHOD AsLogger(nsCycleCollectorLogger * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICYCLECOLLECTORLISTENER \
  nsresult AllTraces(nsICycleCollectorListener **_retval); \
  nsresult GetWantAllTraces(bool *aWantAllTraces); \
  nsresult GetDisableLog(bool *aDisableLog); \
  nsresult SetDisableLog(bool aDisableLog); \
  nsresult GetLogSink(nsICycleCollectorLogSink **aLogSink); \
  nsresult SetLogSink(nsICycleCollectorLogSink *aLogSink); \
  nsresult GetWantAfterProcessing(bool *aWantAfterProcessing); \
  nsresult SetWantAfterProcessing(bool aWantAfterProcessing); \
  nsresult ProcessNext(nsICycleCollectorHandler *aHandler, bool *_retval); \
  nsresult AsLogger(nsCycleCollectorLogger * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICYCLECOLLECTORLISTENER(_to) \
  NS_IMETHOD AllTraces(nsICycleCollectorListener **_retval) override { return _to AllTraces(_retval); } \
  NS_IMETHOD GetWantAllTraces(bool *aWantAllTraces) override { return _to GetWantAllTraces(aWantAllTraces); } \
  NS_IMETHOD GetDisableLog(bool *aDisableLog) override { return _to GetDisableLog(aDisableLog); } \
  NS_IMETHOD SetDisableLog(bool aDisableLog) override { return _to SetDisableLog(aDisableLog); } \
  NS_IMETHOD GetLogSink(nsICycleCollectorLogSink **aLogSink) override { return _to GetLogSink(aLogSink); } \
  NS_IMETHOD SetLogSink(nsICycleCollectorLogSink *aLogSink) override { return _to SetLogSink(aLogSink); } \
  NS_IMETHOD GetWantAfterProcessing(bool *aWantAfterProcessing) override { return _to GetWantAfterProcessing(aWantAfterProcessing); } \
  NS_IMETHOD SetWantAfterProcessing(bool aWantAfterProcessing) override { return _to SetWantAfterProcessing(aWantAfterProcessing); } \
  NS_IMETHOD ProcessNext(nsICycleCollectorHandler *aHandler, bool *_retval) override { return _to ProcessNext(aHandler, _retval); } \
  NS_IMETHOD AsLogger(nsCycleCollectorLogger * * _retval) override { return _to AsLogger(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICYCLECOLLECTORLISTENER(_to) \
  NS_IMETHOD AllTraces(nsICycleCollectorListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllTraces(_retval); } \
  NS_IMETHOD GetWantAllTraces(bool *aWantAllTraces) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWantAllTraces(aWantAllTraces); } \
  NS_IMETHOD GetDisableLog(bool *aDisableLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisableLog(aDisableLog); } \
  NS_IMETHOD SetDisableLog(bool aDisableLog) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisableLog(aDisableLog); } \
  NS_IMETHOD GetLogSink(nsICycleCollectorLogSink **aLogSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLogSink(aLogSink); } \
  NS_IMETHOD SetLogSink(nsICycleCollectorLogSink *aLogSink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogSink(aLogSink); } \
  NS_IMETHOD GetWantAfterProcessing(bool *aWantAfterProcessing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWantAfterProcessing(aWantAfterProcessing); } \
  NS_IMETHOD SetWantAfterProcessing(bool aWantAfterProcessing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWantAfterProcessing(aWantAfterProcessing); } \
  NS_IMETHOD ProcessNext(nsICycleCollectorHandler *aHandler, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessNext(aHandler, _retval); } \
  NS_IMETHOD AsLogger(nsCycleCollectorLogger * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsLogger(_retval); } 


#endif /* __gen_nsICycleCollectorListener_h__ */
