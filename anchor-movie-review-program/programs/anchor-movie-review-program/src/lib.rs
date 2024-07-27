use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo};

pub mod states;
pub use states::*;

declare_id!("3dg9gResFRGUj6RRNcytvdbEXSyriyvSFnrU1ebJV6d1");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
       
        msg!("Movie Comment Counter Account Created");
        let movie_comment_counter = &mut ctx.accounts.movie_comment_counter;
        movie_comment_counter.counter = 0;
        msg!("Counter: {}", movie_comment_counter.counter);

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]],
            ),
            10 * 10 ^ 6,
        )?;
        msg!("Minted tokens");
        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }

    pub fn add_comment(ctx: Context<AddMovieComment>, comment: String) -> Result<()> {
        require!(comment.len() <= MAX_COMMENT_LENGTH, MovieReviewError::CommentTooLong);

        let movie_comment = &mut ctx.accounts.movie_comment;
        let movie_comment_counter = &mut ctx.accounts.movie_comment_counter;

        movie_comment.review = ctx.accounts.movie_review.key();
        movie_comment.commenter = ctx.accounts.initializer.key();
        movie_comment.comment = comment;
        movie_comment_counter.counter += 1;
        movie_comment.count = movie_comment_counter.counter;


        msg!("Comment added");

        Ok(())
    }
}
