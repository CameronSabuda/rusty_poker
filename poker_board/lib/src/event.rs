pub enum BoardModifiedEvent {
    ParticipantAdded {
        participant_id: String,
        participant_name: String,
    },
    ParticipantRemoved {
        participant_id: String,
    },
    ParticipantVoted {
        participant_id: String,
        card_set_id: String,
        card_id: String,
    },
    VotesCleared,
}
