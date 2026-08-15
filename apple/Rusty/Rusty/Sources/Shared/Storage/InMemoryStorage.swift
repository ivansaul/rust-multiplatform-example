//
//  InMemoryStorage.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Foundation

final class InMemoryStorage: KeyValueStorage {
    private var storage: [String: Data] = [:]

    func set<T: Codable>(_ value: T, forKey key: String) {
        storage[key] = try? JSONEncoder().encode(value)
    }

    func get<T: Codable>(_ type: T.Type, forKey key: String) -> T? {
        guard let data = storage[key] else { return nil }
        return try? JSONDecoder().decode(type, from: data)
    }

    func remove(forKey key: String) {
        storage[key] = nil
    }
}
