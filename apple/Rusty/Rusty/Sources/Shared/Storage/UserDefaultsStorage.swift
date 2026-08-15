//
//  UserDefaultsStorage.swift
//  Rusty
//
//  Created by ivansaul on 8/15/26.
//

import Foundation

final class UserDefaultsStorage: KeyValueStorage {
    private let defaults: UserDefaults

    init(defaults: UserDefaults = .standard) {
        self.defaults = defaults
    }

    func set<T: Codable>(_ value: T, forKey key: String) {
        let data = try? JSONEncoder().encode(value)
        defaults.set(data, forKey: key)
    }

    func get<T: Codable>(_ type: T.Type, forKey key: String) -> T? {
        guard let data = defaults.data(forKey: key) else { return nil }
        return try? JSONDecoder().decode(type, from: data)
    }

    func remove(forKey key: String) {
        defaults.removeObject(forKey: key)
    }
}
