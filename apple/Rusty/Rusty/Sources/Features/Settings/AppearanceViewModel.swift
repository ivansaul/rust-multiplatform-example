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
    private let themeKey = "theme"

    var theme: ThemePreference {
        didSet { saveTheme() }
    }

    init() {
        self.theme = Self.loadTheme()
    }

    private func saveTheme() {
        UserDefaults.standard.set(theme.rawValue, forKey: themeKey)
    }

    private static func loadTheme() -> ThemePreference {
        guard
            let rawValue = UserDefaults.standard.string(forKey: "theme"),
            let theme = ThemePreference(rawValue: rawValue)
        else { return .system }

        return theme
    }
}
