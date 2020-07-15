#import "NodeDemoPlugin.h"
#if __has_include(<node_demo/node_demo-Swift.h>)
#import <node_demo/node_demo-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "node_demo-Swift.h"
#endif

@implementation NodeDemoPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftNodeDemoPlugin registerWithRegistrar:registrar];
}
@end
