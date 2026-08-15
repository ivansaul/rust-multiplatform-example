//
//  KeyValueStorage.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Foundation

protocol KeyValueStorage {
    func set<T: Codable>(_ value: T, forKey key: String)
    func get<T: Codable>(_ type: T.Type, forKey key: String) -> T?
    func remove(forKey key: String)
}
