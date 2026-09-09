//
//  StorageManager.swift
//  Rusty
//
//  Created by ivansaul on 9/5/26.
//

import Foundation

final class StorageManager {
    private let fileManager = FileManager.default

    lazy var applicationSupport: URL = fileManager.urls(
        for: .applicationSupportDirectory,
        in: .userDomainMask
    )[0]

    lazy var documents: URL = fileManager.urls(
        for: .documentDirectory,
        in: .userDomainMask
    )[0]

    lazy var caches: URL = fileManager.urls(
        for: .cachesDirectory,
        in: .userDomainMask
    )[0]

    var temporary: URL {
        fileManager.temporaryDirectory
    }
}
