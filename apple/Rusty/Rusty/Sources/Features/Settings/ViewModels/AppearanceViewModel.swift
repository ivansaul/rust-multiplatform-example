//
//  AppearanceViewModel.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import Foundation

@MainActor
@Observable
final class AppearanceViewModel {
    @ObservationIgnored
    private let themeKey = "theme"
    @ObservationIgnored
    private let keyValueStorage: KeyValueStorage

    var theme: ThemePreference = .system {
        didSet { saveTheme() }
    }

    init(keyValueStorage: KeyValueStorage) {
        self.keyValueStorage = keyValueStorage
        self.theme = loadTheme()
    }

    private func saveTheme() {
        keyValueStorage.set(theme.rawValue, forKey: themeKey)
    }

    private func loadTheme() -> ThemePreference {
        guard
            let rawValue = keyValueStorage.get(String.self, forKey: themeKey),
            let theme = ThemePreference(rawValue: rawValue)
        else { return .system }

        return theme
    }
}
