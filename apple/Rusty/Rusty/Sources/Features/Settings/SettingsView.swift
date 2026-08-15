//
//  SettingsView.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import SwiftUI

struct SettingsView: View {
    var body: some View {
        List {
            NavigationLink {
                AppearanceView()
            } label: {
                Label("Appearance", systemImage: "paintpalette.fill")
            }
        }
        .navigationTitle("Settings")
    }
}

#Preview {
    NavigationStack {
        SettingsView()
            .environment(AppearanceViewModel())
    }
}
