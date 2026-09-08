import ProjectDescription

let project = Project(
    name: "Rusty",
    targets: [
        .target(
            name: "Rusty",
            destinations: .iOS,
            product: .app,
            bundleId: "dev.tuist.Rusty",
            deploymentTargets: .iOS("17.0"),
            infoPlist: .extendingDefault(
                with: [
                    "UILaunchScreen": [
                        "UIColorName": "",
                        "UIImageName": "",
                    ],
                ]
            ),
            buildableFolders: [
                "Rusty/Sources",
                "Rusty/Resources",
            ],
            dependencies: [
                .external(name: "Factory"),
                .external(name: "RustyCore"),
                .sdk(name: "SystemConfiguration", type: .framework, status: .required),
            ]
        ),
        .target(
            name: "RustyTests",
            destinations: .iOS,
            product: .unitTests,
            bundleId: "dev.tuist.RustyTests",
            infoPlist: .default,
            buildableFolders: [
                "Rusty/Tests",
            ],
            dependencies: [.target(name: "Rusty")]
        ),
    ]
)
