/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFilePicker.idl
 */

#ifndef __gen_nsIFilePicker_h__
#define __gen_nsIFilePicker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */

class nsIURI; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */


/* starting interface:    nsIFilePickerShownCallback */
#define NS_IFILEPICKERSHOWNCALLBACK_IID_STR "0d79adad-b244-49a5-9997-2a8cad93fc44"

#define NS_IFILEPICKERSHOWNCALLBACK_IID \
  {0x0d79adad, 0xb244, 0x49a5, \
    { 0x99, 0x97, 0x2a, 0x8c, 0xad, 0x93, 0xfc, 0x44 }}

class NS_NO_VTABLE nsIFilePickerShownCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEPICKERSHOWNCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFilePickerShownCallback;

  /* void done (in short aResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Done(int16_t aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFilePickerShownCallback, NS_IFILEPICKERSHOWNCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEPICKERSHOWNCALLBACK \
  NS_IMETHOD Done(int16_t aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEPICKERSHOWNCALLBACK \
  nsresult Done(int16_t aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEPICKERSHOWNCALLBACK(_to) \
  NS_IMETHOD Done(int16_t aResult) override { return _to Done(aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEPICKERSHOWNCALLBACK(_to) \
  NS_IMETHOD Done(int16_t aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Done(aResult); } 


/* starting interface:    nsIFilePicker */
#define NS_IFILEPICKER_IID_STR "9285b984-02d3-46b4-9514-7da8c471a747"

#define NS_IFILEPICKER_IID \
  {0x9285b984, 0x02d3, 0x46b4, \
    { 0x95, 0x14, 0x7d, 0xa8, 0xc4, 0x71, 0xa7, 0x47 }}

class NS_NO_VTABLE nsIFilePicker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILEPICKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFilePicker;

  enum {
    modeOpen = 0,
    modeSave = 1,
    modeGetFolder = 2,
    modeOpenMultiple = 3,
    returnOK = 0,
    returnCancel = 1,
    returnReplace = 2,
    filterAll = 1,
    filterHTML = 2,
    filterText = 4,
    filterImages = 8,
    filterXML = 16,
    filterXUL = 32,
    filterApps = 64,
    filterAllowURLs = 128,
    filterAudio = 256,
    filterVideo = 512,
    captureNone = 0,
    captureDefault = 1,
    captureUser = 2,
    captureEnv = 3
  };

  /* void init (in mozIDOMWindowProxy parent, in AString title, in short mode); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, int16_t mode) = 0;

  /* void appendFilters (in long filterMask); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendFilters(int32_t filterMask) = 0;

  /* void appendFilter (in AString title, in AString filter); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendFilter(const nsAString& title, const nsAString& filter) = 0;

  /* void appendRawFilter (in AString filter); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendRawFilter(const nsAString& filter) = 0;

  /* attribute AString defaultString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultString(nsAString& aDefaultString) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultString(const nsAString& aDefaultString) = 0;

  /* attribute AString defaultExtension; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultExtension(nsAString& aDefaultExtension) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultExtension(const nsAString& aDefaultExtension) = 0;

  /* attribute long filterIndex; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFilterIndex(int32_t *aFilterIndex) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFilterIndex(int32_t aFilterIndex) = 0;

  /* attribute nsIFile displayDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisplayDirectory(nsIFile **aDisplayDirectory) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisplayDirectory(nsIFile *aDisplayDirectory) = 0;

  /* attribute AString displaySpecialDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisplaySpecialDirectory(nsAString& aDisplaySpecialDirectory) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDisplaySpecialDirectory(const nsAString& aDisplaySpecialDirectory) = 0;

  /* readonly attribute nsIFile file; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFile(nsIFile **aFile) = 0;

  /* readonly attribute nsIURI fileURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileURL(nsIURI **aFileURL) = 0;

  /* readonly attribute nsISimpleEnumerator files; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFiles(nsISimpleEnumerator **aFiles) = 0;

  /* readonly attribute nsISupports domFileOrDirectory; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomFileOrDirectory(nsISupports **aDomFileOrDirectory) = 0;

  /* readonly attribute nsISimpleEnumerator domFileOrDirectoryEnumerator; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomFileOrDirectoryEnumerator(nsISimpleEnumerator **aDomFileOrDirectoryEnumerator) = 0;

  /* attribute boolean addToRecentDocs; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAddToRecentDocs(bool *aAddToRecentDocs) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAddToRecentDocs(bool aAddToRecentDocs) = 0;

  /* void open (in nsIFilePickerShownCallback aFilePickerShownCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Open(nsIFilePickerShownCallback *aFilePickerShownCallback) = 0;

  /* readonly attribute short mode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMode(int16_t *aMode) = 0;

  /* attribute AString okButtonLabel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOkButtonLabel(nsAString& aOkButtonLabel) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetOkButtonLabel(const nsAString& aOkButtonLabel) = 0;

  /* attribute short capture; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCapture(int16_t *aCapture) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCapture(int16_t aCapture) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFilePicker, NS_IFILEPICKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILEPICKER \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, int16_t mode) override; \
  NS_IMETHOD AppendFilters(int32_t filterMask) override; \
  NS_IMETHOD AppendFilter(const nsAString& title, const nsAString& filter) override; \
  NS_IMETHOD AppendRawFilter(const nsAString& filter) override; \
  NS_IMETHOD GetDefaultString(nsAString& aDefaultString) override; \
  NS_IMETHOD SetDefaultString(const nsAString& aDefaultString) override; \
  NS_IMETHOD GetDefaultExtension(nsAString& aDefaultExtension) override; \
  NS_IMETHOD SetDefaultExtension(const nsAString& aDefaultExtension) override; \
  NS_IMETHOD GetFilterIndex(int32_t *aFilterIndex) override; \
  NS_IMETHOD SetFilterIndex(int32_t aFilterIndex) override; \
  NS_IMETHOD GetDisplayDirectory(nsIFile **aDisplayDirectory) override; \
  NS_IMETHOD SetDisplayDirectory(nsIFile *aDisplayDirectory) override; \
  NS_IMETHOD GetDisplaySpecialDirectory(nsAString& aDisplaySpecialDirectory) override; \
  NS_IMETHOD SetDisplaySpecialDirectory(const nsAString& aDisplaySpecialDirectory) override; \
  NS_IMETHOD GetFile(nsIFile **aFile) override; \
  NS_IMETHOD GetFileURL(nsIURI **aFileURL) override; \
  NS_IMETHOD GetFiles(nsISimpleEnumerator **aFiles) override; \
  NS_IMETHOD GetDomFileOrDirectory(nsISupports **aDomFileOrDirectory) override; \
  NS_IMETHOD GetDomFileOrDirectoryEnumerator(nsISimpleEnumerator **aDomFileOrDirectoryEnumerator) override; \
  NS_IMETHOD GetAddToRecentDocs(bool *aAddToRecentDocs) override; \
  NS_IMETHOD SetAddToRecentDocs(bool aAddToRecentDocs) override; \
  NS_IMETHOD Open(nsIFilePickerShownCallback *aFilePickerShownCallback) override; \
  NS_IMETHOD GetMode(int16_t *aMode) override; \
  NS_IMETHOD GetOkButtonLabel(nsAString& aOkButtonLabel) override; \
  NS_IMETHOD SetOkButtonLabel(const nsAString& aOkButtonLabel) override; \
  NS_IMETHOD GetCapture(int16_t *aCapture) override; \
  NS_IMETHOD SetCapture(int16_t aCapture) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILEPICKER \
  nsresult Init(mozIDOMWindowProxy *parent, const nsAString& title, int16_t mode); \
  nsresult AppendFilters(int32_t filterMask); \
  nsresult AppendFilter(const nsAString& title, const nsAString& filter); \
  nsresult AppendRawFilter(const nsAString& filter); \
  nsresult GetDefaultString(nsAString& aDefaultString); \
  nsresult SetDefaultString(const nsAString& aDefaultString); \
  nsresult GetDefaultExtension(nsAString& aDefaultExtension); \
  nsresult SetDefaultExtension(const nsAString& aDefaultExtension); \
  nsresult GetFilterIndex(int32_t *aFilterIndex); \
  nsresult SetFilterIndex(int32_t aFilterIndex); \
  nsresult GetDisplayDirectory(nsIFile **aDisplayDirectory); \
  nsresult SetDisplayDirectory(nsIFile *aDisplayDirectory); \
  nsresult GetDisplaySpecialDirectory(nsAString& aDisplaySpecialDirectory); \
  nsresult SetDisplaySpecialDirectory(const nsAString& aDisplaySpecialDirectory); \
  nsresult GetFile(nsIFile **aFile); \
  nsresult GetFileURL(nsIURI **aFileURL); \
  nsresult GetFiles(nsISimpleEnumerator **aFiles); \
  nsresult GetDomFileOrDirectory(nsISupports **aDomFileOrDirectory); \
  nsresult GetDomFileOrDirectoryEnumerator(nsISimpleEnumerator **aDomFileOrDirectoryEnumerator); \
  nsresult GetAddToRecentDocs(bool *aAddToRecentDocs); \
  nsresult SetAddToRecentDocs(bool aAddToRecentDocs); \
  nsresult Open(nsIFilePickerShownCallback *aFilePickerShownCallback); \
  nsresult GetMode(int16_t *aMode); \
  nsresult GetOkButtonLabel(nsAString& aOkButtonLabel); \
  nsresult SetOkButtonLabel(const nsAString& aOkButtonLabel); \
  nsresult GetCapture(int16_t *aCapture); \
  nsresult SetCapture(int16_t aCapture); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILEPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, int16_t mode) override { return _to Init(parent, title, mode); } \
  NS_IMETHOD AppendFilters(int32_t filterMask) override { return _to AppendFilters(filterMask); } \
  NS_IMETHOD AppendFilter(const nsAString& title, const nsAString& filter) override { return _to AppendFilter(title, filter); } \
  NS_IMETHOD AppendRawFilter(const nsAString& filter) override { return _to AppendRawFilter(filter); } \
  NS_IMETHOD GetDefaultString(nsAString& aDefaultString) override { return _to GetDefaultString(aDefaultString); } \
  NS_IMETHOD SetDefaultString(const nsAString& aDefaultString) override { return _to SetDefaultString(aDefaultString); } \
  NS_IMETHOD GetDefaultExtension(nsAString& aDefaultExtension) override { return _to GetDefaultExtension(aDefaultExtension); } \
  NS_IMETHOD SetDefaultExtension(const nsAString& aDefaultExtension) override { return _to SetDefaultExtension(aDefaultExtension); } \
  NS_IMETHOD GetFilterIndex(int32_t *aFilterIndex) override { return _to GetFilterIndex(aFilterIndex); } \
  NS_IMETHOD SetFilterIndex(int32_t aFilterIndex) override { return _to SetFilterIndex(aFilterIndex); } \
  NS_IMETHOD GetDisplayDirectory(nsIFile **aDisplayDirectory) override { return _to GetDisplayDirectory(aDisplayDirectory); } \
  NS_IMETHOD SetDisplayDirectory(nsIFile *aDisplayDirectory) override { return _to SetDisplayDirectory(aDisplayDirectory); } \
  NS_IMETHOD GetDisplaySpecialDirectory(nsAString& aDisplaySpecialDirectory) override { return _to GetDisplaySpecialDirectory(aDisplaySpecialDirectory); } \
  NS_IMETHOD SetDisplaySpecialDirectory(const nsAString& aDisplaySpecialDirectory) override { return _to SetDisplaySpecialDirectory(aDisplaySpecialDirectory); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } \
  NS_IMETHOD GetFileURL(nsIURI **aFileURL) override { return _to GetFileURL(aFileURL); } \
  NS_IMETHOD GetFiles(nsISimpleEnumerator **aFiles) override { return _to GetFiles(aFiles); } \
  NS_IMETHOD GetDomFileOrDirectory(nsISupports **aDomFileOrDirectory) override { return _to GetDomFileOrDirectory(aDomFileOrDirectory); } \
  NS_IMETHOD GetDomFileOrDirectoryEnumerator(nsISimpleEnumerator **aDomFileOrDirectoryEnumerator) override { return _to GetDomFileOrDirectoryEnumerator(aDomFileOrDirectoryEnumerator); } \
  NS_IMETHOD GetAddToRecentDocs(bool *aAddToRecentDocs) override { return _to GetAddToRecentDocs(aAddToRecentDocs); } \
  NS_IMETHOD SetAddToRecentDocs(bool aAddToRecentDocs) override { return _to SetAddToRecentDocs(aAddToRecentDocs); } \
  NS_IMETHOD Open(nsIFilePickerShownCallback *aFilePickerShownCallback) override { return _to Open(aFilePickerShownCallback); } \
  NS_IMETHOD GetMode(int16_t *aMode) override { return _to GetMode(aMode); } \
  NS_IMETHOD GetOkButtonLabel(nsAString& aOkButtonLabel) override { return _to GetOkButtonLabel(aOkButtonLabel); } \
  NS_IMETHOD SetOkButtonLabel(const nsAString& aOkButtonLabel) override { return _to SetOkButtonLabel(aOkButtonLabel); } \
  NS_IMETHOD GetCapture(int16_t *aCapture) override { return _to GetCapture(aCapture); } \
  NS_IMETHOD SetCapture(int16_t aCapture) override { return _to SetCapture(aCapture); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILEPICKER(_to) \
  NS_IMETHOD Init(mozIDOMWindowProxy *parent, const nsAString& title, int16_t mode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(parent, title, mode); } \
  NS_IMETHOD AppendFilters(int32_t filterMask) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendFilters(filterMask); } \
  NS_IMETHOD AppendFilter(const nsAString& title, const nsAString& filter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendFilter(title, filter); } \
  NS_IMETHOD AppendRawFilter(const nsAString& filter) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendRawFilter(filter); } \
  NS_IMETHOD GetDefaultString(nsAString& aDefaultString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultString(aDefaultString); } \
  NS_IMETHOD SetDefaultString(const nsAString& aDefaultString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultString(aDefaultString); } \
  NS_IMETHOD GetDefaultExtension(nsAString& aDefaultExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultExtension(aDefaultExtension); } \
  NS_IMETHOD SetDefaultExtension(const nsAString& aDefaultExtension) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultExtension(aDefaultExtension); } \
  NS_IMETHOD GetFilterIndex(int32_t *aFilterIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilterIndex(aFilterIndex); } \
  NS_IMETHOD SetFilterIndex(int32_t aFilterIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFilterIndex(aFilterIndex); } \
  NS_IMETHOD GetDisplayDirectory(nsIFile **aDisplayDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayDirectory(aDisplayDirectory); } \
  NS_IMETHOD SetDisplayDirectory(nsIFile *aDisplayDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplayDirectory(aDisplayDirectory); } \
  NS_IMETHOD GetDisplaySpecialDirectory(nsAString& aDisplaySpecialDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplaySpecialDirectory(aDisplaySpecialDirectory); } \
  NS_IMETHOD SetDisplaySpecialDirectory(const nsAString& aDisplaySpecialDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDisplaySpecialDirectory(aDisplaySpecialDirectory); } \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } \
  NS_IMETHOD GetFileURL(nsIURI **aFileURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileURL(aFileURL); } \
  NS_IMETHOD GetFiles(nsISimpleEnumerator **aFiles) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFiles(aFiles); } \
  NS_IMETHOD GetDomFileOrDirectory(nsISupports **aDomFileOrDirectory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomFileOrDirectory(aDomFileOrDirectory); } \
  NS_IMETHOD GetDomFileOrDirectoryEnumerator(nsISimpleEnumerator **aDomFileOrDirectoryEnumerator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomFileOrDirectoryEnumerator(aDomFileOrDirectoryEnumerator); } \
  NS_IMETHOD GetAddToRecentDocs(bool *aAddToRecentDocs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddToRecentDocs(aAddToRecentDocs); } \
  NS_IMETHOD SetAddToRecentDocs(bool aAddToRecentDocs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAddToRecentDocs(aAddToRecentDocs); } \
  NS_IMETHOD Open(nsIFilePickerShownCallback *aFilePickerShownCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aFilePickerShownCallback); } \
  NS_IMETHOD GetMode(int16_t *aMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMode(aMode); } \
  NS_IMETHOD GetOkButtonLabel(nsAString& aOkButtonLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOkButtonLabel(aOkButtonLabel); } \
  NS_IMETHOD SetOkButtonLabel(const nsAString& aOkButtonLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOkButtonLabel(aOkButtonLabel); } \
  NS_IMETHOD GetCapture(int16_t *aCapture) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCapture(aCapture); } \
  NS_IMETHOD SetCapture(int16_t aCapture) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCapture(aCapture); } 


#endif /* __gen_nsIFilePicker_h__ */
