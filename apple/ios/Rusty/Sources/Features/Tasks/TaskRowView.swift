//
//  TaskRowView.swift
//  Rusty
//
//  Created by ivansaul on 9/5/26.
//

import RustyCore
import SwiftUI

struct TaskRowView: View {
    let task: TaskItem
    let onToggle: () -> Void
    let onDelete: () -> Void

    var body: some View {
        HStack {
            Button(action: onToggle) {
                Image(systemName: task.completed ? "checkmark.seal.fill" : "checkmark.seal")
                    .foregroundColor(.blue)
            }

            Text(task.title)
                .strikethrough(task.completed, color: .secondary)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .swipeActions(edge: .trailing) {
            Button(role: .destructive, action: onDelete) {
                Image(systemName: "trash")
            }
        }
    }
}

#Preview {
    TaskRowView(
        task: TaskItem(
            id: UUID().uuidString,
            title: "Some Task",
            completed: false
        ),
        onToggle: {},
        onDelete: {}
    )
}
